use bitvec::{
    field::BitField,
    prelude::{BitArray, Msb0},
};
use bmp::Pixel;

#[derive(Debug, Clone, Copy)]
enum Color {
    White,
    Light,
    Dark,
    Black,
}

fn load_tile(img: &bmp::Image, col: usize, row: usize) -> Vec<Color> {
    let mut tile = Vec::with_capacity(64);
    for y in 0..8 {
        for x in 0..8 {
            let py = (col * 8) + y;
            let px = (row * 8) + x;
            let pixel = img.get_pixel(px as u32, py as u32);
            tile.push(match pixel {
                Pixel { r: 0, g: 0, b: 0 } => Color::Black,
                Pixel {
                    r: 85,
                    g: 85,
                    b: 85,
                } => Color::Dark,
                Pixel {
                    r: 170,
                    g: 170,
                    b: 170,
                } => Color::Light,
                Pixel {
                    r: 255,
                    g: 255,
                    b: 255,
                } => Color::White,
                _ => panic!("{:?}", pixel),
            });
        }
    }
    tile
}

fn load_tilesheet(path: &str, col_size: usize, row_size: usize) -> Vec<Vec<Color>> {
    let img = bmp::open(path).unwrap_or_else(|e| panic!("Failed to open: {}", e));
    let mut tiles = Vec::with_capacity(col_size * row_size);
    for col in 0..col_size {
        for row in 0..row_size {
            tiles.push(load_tile(&img, col, row));
        }
    }
    tiles
}

fn convert_tilesheet_to_hex(tilesheet: Vec<Vec<Color>>) -> Vec<Vec<u8>> {
    let mut tiles = Vec::with_capacity(16);
    for tile in tilesheet.iter() {
        let mut line_vec = Vec::with_capacity(64);
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
            line_vec.push(low.load::<u8>());
            line_vec.push(high.load::<u8>());
        }
        tiles.push(line_vec);
    }
    tiles
}

fn create_static_tilesheet() {
    let width = 8;
    let height = 8;
    let tilesheet = load_tilesheet("assets/dungeon.bmp", height, width);
    let tiles = convert_tilesheet_to_hex(tilesheet);
    let result = tiles
        .iter()
        .flat_map(|tile| tile.iter().collect::<Vec<_>>())
        .enumerate()
        .map(|(idx, tile_str)| format!("    // TILE{:02}\n    {},", idx, tile_str))
        .collect::<Vec<_>>()
        .join("\n");

    println!("pub static TILES: [u8; 1024] = [\n{}\n];", result);
}

fn main() {
    create_static_tilesheet();
}
