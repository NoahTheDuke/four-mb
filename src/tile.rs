#![allow(dead_code)]

use crate::part::*;
use macroquad::color::Color as mColor;
use std::convert::TryInto;
use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    tl: Part,
    tr: Part,
    bl: Part,
    br: Part,
}

impl Tile {
    pub fn new(tl: Part, tr: Part, bl: Part, br: Part) -> Tile {
        Tile { tl, tr, bl, br }
    }

    pub fn from_bytes(tl: &[u8; 16], tr: &[u8; 16], bl: &[u8; 16], br: &[u8; 16]) -> Tile {
        Tile {
            tl: Part::new(tl),
            tr: Part::new(tr),
            bl: Part::new(bl),
            br: Part::new(br),
        }
    }
}

impl Index<usize> for Tile {
    type Output = mColor;

    fn index(&self, index: usize) -> &Self::Output {
        let x = index % 16;
        let y = index / 16;
        let left = x < 8;
        let top = y < 8;

        match (left, top) {
            (true, true) => {
                let idx = (y * 8) + x;
                &self.tl[idx]
            }
            (false, true) => {
                let idx = (y * 8) + (x - 8);
                &self.tr[idx]
            }
            (true, false) => {
                let idx = ((y - 8) * 8) + x;
                &self.bl[idx]
            }
            (false, false) => {
                let idx = ((y - 8) * 8) + (x - 8);
                &self.br[idx]
            }
        }
    }
}

pub struct TileIterator<'a> {
    tile: &'a Tile,
    index: usize,
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = mColor;
    fn next(&mut self) -> Option<mColor> {
        if self.index > 255 {
            None
        } else {
            let result = self.tile[self.index];
            self.index += 1;
            Some(result)
        }
    }
}

impl<'a> IntoIterator for &'a Tile {
    type Item = mColor;
    type IntoIter = TileIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TileIterator {
            tile: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macroquad::color::{BLACK, DARKGRAY, LIGHTGRAY, WHITE};

    #[test]
    fn test_correct_part() {
        let tl_part = Part::new(&[0u8; 16]);
        let tr_part = Part::new(&[
            255u8, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        ]);
        let bl_part = Part::new(&[
            0u8, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255,
        ]);
        let br_part = Part::new(&[255u8; 16]);
        let tile = Tile::new(tl_part, tr_part, bl_part, br_part);

        assert_eq!(tile[0], WHITE);
        assert_eq!(tile[9], LIGHTGRAY);
        assert_eq!(tile[128], DARKGRAY);
        assert_eq!(tile[136], BLACK);
    }
}
