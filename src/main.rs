#[allow(dead_code)]

use macroquad::prelude::*;
use bitvec::prelude::*;
use std::iter::FromIterator;

static PALETTE: [[u8; 2]; 2] = [
    [0, 1],
    [2, 3],
];

static WINDOW: [u8; 16] = [
    0x7C, 0x7C, 0x00, 0xC6, 0xC6, 0x00, 0x00, 0xFE, 0xC6, 0xC6, 0x00, 0xC6, 0xC6, 0x00, 0x00, 0x00,
];

#[macroquad::main("Zelda-like")]
async fn main() {
    let mut chunks = WINDOW.chunks(2);

    let mut tile = String::with_capacity(64);

    while let Some(cur) = chunks.next() {
        let low = *&cur[0].view_bits::<Msb0>();
        let high = *&cur[1].view_bits::<Msb0>();

        for (h, l) in high.iter().by_val().zip(low.iter().by_val()) {
            let color = PALETTE[h as usize][l as usize];
            tile.push(
                match color {
                    0 => '.',
                    c => char::from_digit(c as u32, 10).unwrap(),
                }
            )
        }
    }

    for line in tile.chars().into_iter().collect::<Vec::<_>>().chunks(8) {
        println!("{}", String::from_iter(line.iter().collect::<Vec<_>>()));
    }
}
