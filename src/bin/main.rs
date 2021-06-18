#![allow(dead_code, unused_variables)]

use lib::prelude::*;
use macroquad::prelude::*;

static PIXEL_SIZE: f32 = 3.0;
static COLUMNS: usize = 8;
static ROWS: usize = 10;

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
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
    let texture = Texture2D::from_image(&image);
    let floor_tile = *TILESET.get(&4).unwrap();

    loop {
        clear_background(WHITE);

        let w = image.width();
        let h = image.height();

        let room = ROOMS.get("3-7").unwrap();

        for col in 0..COLUMNS {
            for row in 0..ROWS {
                if let Some(tile) = room.get_tile(row, col) {
                    for (y, rows) in tile.into_iter().collect::<Vec<_>>().chunks(16).enumerate() {
                        for (x, px) in rows.iter().enumerate() {
                            image.set_pixel(((row * 16) + x) as u32, ((col * 16) + y) as u32, *px);
                        }
                    }
                } else {
                    panic!("{:?} {:?}", row, col);
                }
            }
        }

        texture.update(&image);

        draw_texture_ex(
            texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        next_frame().await
    }
}
