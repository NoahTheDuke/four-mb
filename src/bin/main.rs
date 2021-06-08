use bitvec::{prelude::Msb0, view::BitView};
use lib::prelude::TILES;
use macroquad::prelude::*;

#[rustfmt::skip]
static PALETTE: [[macroquad::color::Color; 2]; 2] = [
    [WHITE, LIGHTGRAY],
    [DARKGRAY, BLACK],
];

fn render_tile(tile: &[u8]) -> [macroquad::color::Color; 256] {
    let mut tile_arr = [WHITE; 256];

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

#[macroquad::main(window_conf)]
async fn main() {
    let origin = screen_width() / 2.0;

    let tile = render_tile(&TILES[0..32]);
    loop {
        clear_background(WHITE);

        for (col_idx, rows) in tile.iter().collect::<Vec<_>>().chunks(8).enumerate() {
            for (row_idx, px) in rows.iter().enumerate() {
                let x = (row_idx as f32) * PIXEL_SIZE + origin;
                let y = (col_idx as f32) * PIXEL_SIZE + origin;
                draw_rectangle(x, y, PIXEL_SIZE, PIXEL_SIZE, **px);
            }
        }

        next_frame().await
    }
}
