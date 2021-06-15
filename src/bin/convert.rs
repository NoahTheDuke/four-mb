use bitvec::{
    field::BitField,
    prelude::{BitArray, Msb0},
};
use macroquad::{prelude::ImageFormat, texture::Image};
use std::ffi::OsStr;
use std::{fs, io};

#[derive(Debug, Clone, Copy)]
enum Color {
    White,
    Light,
    Dark,
    Black,
}

fn load_tile(img: &Image, col: usize, row: usize) -> Vec<Color> {
    (0..8)
        .flat_map(|y| {
            (0..8).map(move |x| {
                let py = (row * 8) + y;
                let px = (col * 8) + x;
                let pixel = img.get_pixel(px as u32, py as u32);
                if pixel.r < 0.2 {
                    Color::Black
                } else if pixel.r < 0.4 {
                    Color::Dark
                } else if pixel.r < 0.7 {
                    Color::Light
                } else {
                    Color::White
                }
            })
        })
        .collect()
}

fn load_tilesheet(path: &str) -> Result<Vec<Vec<Color>>, io::Error> {
    let img_bytes = fs::read(path)?;
    let img = Image::from_file_with_format(&img_bytes[..], Some(ImageFormat::Png));
    let width = img.width() / 8;
    let height = img.height() / 8;
    let mut tiles = Vec::with_capacity(width * height);
    for row in 0..height {
        for col in 0..width {
            tiles.push(load_tile(&img, col, row));
        }
    }
    Ok(tiles)
}

fn convert_tilesheet_to_hex(tilesheet: Vec<Vec<Color>>) -> Vec<u8> {
    let mut tiles = Vec::new();
    for tile in tilesheet.iter() {
        for line in tile.chunks(8) {
            let mut low: BitArray<Msb0, [u8; 1]> = BitArray::zeroed();
            let mut high: BitArray<Msb0, [u8; 1]> = BitArray::zeroed();

            for (idx, px) in line.iter().enumerate() {
                match px {
                    Color::White => {
                        *low.get_mut(idx).unwrap() = false;
                        *high.get_mut(idx).unwrap() = false;
                    }
                    Color::Light => {
                        *low.get_mut(idx).unwrap() = true;
                        *high.get_mut(idx).unwrap() = false;
                    }
                    Color::Dark => {
                        *low.get_mut(idx).unwrap() = false;
                        *high.get_mut(idx).unwrap() = true;
                    }
                    Color::Black => {
                        *low.get_mut(idx).unwrap() = true;
                        *high.get_mut(idx).unwrap() = true;
                    }
                }
            }
            tiles.push(low.load::<u8>());
            tiles.push(high.load::<u8>());
        }
    }
    tiles
}

fn create_static_tilesheet() -> io::Result<()> {
    let entries = fs::read_dir("assets")?
        .map(|res| res.map(|e| e.path()))
        .filter(|res| {
            res.as_ref().map_or(false, |path| {
                path.extension().unwrap_or(OsStr::new("")) == "png"
            })
        });

    for entry in entries {
        if let Ok(mut path) = entry {
            let path_str = path.to_str().unwrap();
            if let Ok(tilesheet) = load_tilesheet(path_str) {
                let tiles = convert_tilesheet_to_hex(tilesheet);
                path.set_extension("2bpp");
                fs::write(path, tiles).expect("Unable to write file");
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    create_static_tilesheet()
}
