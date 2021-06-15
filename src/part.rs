#![allow(dead_code)]

use bitvec::{prelude::Msb0, view::BitView};
use macroquad::color::{Color as mColor, BLACK, DARKGRAY, LIGHTGRAY, WHITE};
use std::convert::TryInto;
use std::ops::Index;

#[rustfmt::skip]
static PALETTE: [[mColor; 2]; 2] = [
    [WHITE, LIGHTGRAY],
    [DARKGRAY, BLACK],
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Part {
    pub points: [mColor; 64],
}

impl Eq for Part {}

impl Part {
    pub fn new(bytes: &[u8; 16]) -> Part {
        let mut tile_arr = [WHITE; 64];

        for (y_idx, cur) in bytes.chunks(2).enumerate() {
            let low = *&cur[0].view_bits::<Msb0>();
            let high = *&cur[1].view_bits::<Msb0>();

            for (x_idx, (h, l)) in high.iter().by_val().zip(low.iter().by_val()).enumerate() {
                let idx = (y_idx * 8) + x_idx;
                tile_arr[idx] = PALETTE[h as usize][l as usize];
            }
        }
        Part { points: tile_arr }
    }

    pub fn get_line(&self, line: usize) -> [mColor; 8] {
        assert!(line <= 7);
        self.points[line..(line + 1) * 8].try_into().unwrap()
    }
}

impl Index<usize> for Part {
    type Output = mColor;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl IntoIterator for Part {
    type Item = mColor;
    type IntoIter = std::array::IntoIter<mColor, 64>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new(self.points)
    }
}

impl IntoIterator for &Part {
    type Item = mColor;
    type IntoIter = std::array::IntoIter<mColor, 64>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new(self.points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line() {
        let part = Part::new(&[0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            part.get_line(0),
            [WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE]
        );
    }

    #[test]
    #[should_panic]
    fn test_get_line_gt_7() {
        let part = Part::new(&[0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        part.get_line(8);
    }

    #[test]
    fn test_into_iterator() {
        let part = Part::new(&[0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let mut arr = vec![];
        for p in part {
            arr.push(p);
        }
        assert_eq!(arr, [WHITE; 64]);
    }
}
