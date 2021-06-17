use crate::tile::Tile;
use crate::tileset::TILESET;

pub struct Map {
    pub name: String,
    pub rooms: Vec<Room>,
}

pub struct WallShape {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Default for WallShape {
    fn default() -> Self {
        WallShape {
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }
}

/// Instruction
/// P: Point (x, y, tile index)
/// L: Line (LD(size), x, y, tile index)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Inst {
    pub x: usize,
    pub y: usize,
    pub tile_idx: usize,
    pub vector: Option<u8>,
}

pub const fn point(x: usize, y: usize, tile_idx: usize) -> Inst {
    Inst {
        x,
        y,
        tile_idx,
        vector: None,
    }
}

pub const fn line(x: usize, y: usize, tile_idx: usize, vector: u8) -> Inst {
    Inst {
        x,
        y,
        tile_idx,
        vector: Some(vector),
    }
}

fn build_room(instructions: &Vec<Inst>, tiles: &mut Vec<Tile>) {
    for Inst {
        x,
        y,
        tile_idx,
        vector,
    } in instructions
    {
        let idx = (y * 10) + x;
        let got_tile = *TILESET.get(&tile_idx).unwrap();

        if let Some(vector) = vector {
            let direction = vector & 0xF0;
            let length = vector & 0x0F;
            assert!(length > 1, "length must be greater than 1");
            match direction {
                0x80 => {
                    let mut counter = 0usize;
                    while counter < length as usize {
                        tiles[idx + counter] = got_tile;
                        counter += 1;
                    }
                }
                0xc0 => {
                    let mut counter = 0usize;
                    while counter < length as usize {
                        tiles[idx + (counter * 10)] = got_tile;
                        counter += 1;
                    }
                }
                _ => panic!(
                    "wrong direction bro 0x{:02x} 0x{:02x?} 0x{:02x?}",
                    vector,
                    direction,
                    length
                ),
            }
        } else {
            tiles[idx] = got_tile;
        }
    }
}

pub struct Room {
    pub wall_shape: WallShape,
    pub floor_tile: Tile,
    pub instructions: Vec<Inst>,
    pub tiles: Vec<Tile>,
}

impl Room {
    pub fn new(wall_shape: WallShape, floor_tile_idx: usize, instructions: Vec<Inst>) -> Room {
        let floor_tile = *TILESET.get(&floor_tile_idx).unwrap();

        let mut tiles = vec![floor_tile; 80];
        if wall_shape.top {
            for idx in 1..9 {
                tiles[idx] = *TILESET.get(&7).unwrap();
            }
        }
        if wall_shape.bottom {
            for idx in 71..79 {
                tiles[idx] = *TILESET.get(&8).unwrap();
            }
        }
        if wall_shape.left {
            for idx in vec![10, 20, 30, 40, 50, 60] {
                tiles[idx] = *TILESET.get(&9).unwrap();
            }
        }
        if wall_shape.right {
            for idx in vec![19, 29, 39, 49, 59, 69] {
                tiles[idx] = *TILESET.get(&10).unwrap();
            }
        }
        if wall_shape.top {
            if wall_shape.left {
                tiles[0] = *TILESET.get(&15).unwrap();
            }
            if wall_shape.right {
                tiles[9] = *TILESET.get(&16).unwrap();
            }
        }
        if wall_shape.bottom {
            if wall_shape.left {
                tiles[70] = *TILESET.get(&17).unwrap();
            }
            if wall_shape.right {
                tiles[79] = *TILESET.get(&18).unwrap();
            }
        }

        build_room(&instructions, &mut tiles);

        Room {
            wall_shape,
            floor_tile,
            instructions,
            tiles,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x > 9 || y > 7 {
            return None;
        }
        let idx = (y * 10) + x;
        Some(self.tiles[idx])
    }
}
