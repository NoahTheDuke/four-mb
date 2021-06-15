#![allow(dead_code, unused_variables)]

use bitvec::{prelude::Msb0, view::BitView};
use lib::prelude::*;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;

#[rustfmt::skip]
static PALETTE: [[Color; 2]; 2] = [
    [WHITE, LIGHTGRAY],
    [DARKGRAY, BLACK],
];

fn render_tile(tile: Vec<u8>) -> [Color; 64] {
    let mut tile_arr = [WHITE; 64];

    for (row_idx, cur) in tile.chunks(2).enumerate() {
        let low = *&cur[0].view_bits::<Msb0>();
        let high = *&cur[1].view_bits::<Msb0>();

        for (col_idx, (h, l)) in high.iter().by_val().zip(low.iter().by_val()).enumerate() {
            let idx = (row_idx * 8) + col_idx;
            tile_arr[idx] = PALETTE[h as usize][l as usize];
        }
    }
    tile_arr
}

static PIXEL_SIZE: f32 = 3.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Zelda-like".to_owned(),
        fullscreen: false,
        window_width: 160 * PIXEL_SIZE as i32,
        window_height: 144 * PIXEL_SIZE as i32,
        ..Default::default()
    }
}

fn build_tileset(parts_array: &[u8]) -> HashMap<usize, Part> {
    parts_array
        .chunks(16)
        .map(|chunk| Part::new(chunk.try_into().unwrap()))
        .enumerate()
        .collect()
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut buffer = vec![WHITE; w * h];
    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    let doors_tileset = build_tileset(DOORS);

    let door_tile = Tile::new(
        *doors_tileset.get(&0).unwrap(),
        *doors_tileset.get(&1).unwrap(),
        *doors_tileset.get(&2).unwrap(),
        *doors_tileset.get(&3).unwrap(),
    );

    loop {
        clear_background(WHITE);

        let w = image.width();
        let h = image.height();

        for (y, rows) in door_tile.into_iter().collect::<Vec<_>>().chunks(16).enumerate() {
            for (x, px) in rows.iter().enumerate() {
                image.set_pixel(x as u32, y as u32, *px);
            }
        }

        texture.update(&image);

        draw_texture(texture, 0., 0., WHITE);

        next_frame().await
    }
}
