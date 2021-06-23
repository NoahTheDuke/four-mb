use crate::tile::Tile;
use crate::tileset::TILESET;

pub struct Map {
    pub name: String,
    pub rooms: Vec<Room>,
}

/// top, right, bottom, left
#[derive(Debug)]
pub struct WallShape(pub bool, pub bool, pub bool, pub bool);

impl Default for WallShape {
    fn default() -> Self {
        WallShape(true, true, true, true)
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

pub const fn pt(x: usize, y: usize, tile_idx: usize) -> Inst {
    Inst {
        x,
        y,
        tile_idx,
        vector: None,
    }
}

pub const fn ln(x: usize, y: usize, tile_idx: usize, vector: u8) -> Inst {
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
            let direction = vector / 10;
            let length = vector % 10;
            assert!(length > 1, "length must be greater than 1");
            match direction {
                8 => {
                    let mut counter = 0usize;
                    while counter < length as usize {
                        tiles[idx + counter] = got_tile;
                        counter += 1;
                    }
                }
                9 => {
                    let mut counter = 0usize;
                    while counter < length as usize {
                        tiles[idx + (counter * 10)] = got_tile;
                        counter += 1;
                    }
                }
                _ => panic!(
                    "wrong direction bro {:?} {:?} {:?}",
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

#[derive(Debug)]
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
        if wall_shape.0 {
            for idx in 1..9 {
                tiles[idx] = *TILESET.get(&7).unwrap();
            }
        }
        if wall_shape.2 {
            for idx in 71..79 {
                tiles[idx] = *TILESET.get(&8).unwrap();
            }
        }
        if wall_shape.3 {
            for idx in vec![10, 20, 30, 40, 50, 60] {
                tiles[idx] = *TILESET.get(&9).unwrap();
            }
        }
        if wall_shape.1 {
            for idx in vec![19, 29, 39, 49, 59, 69] {
                tiles[idx] = *TILESET.get(&10).unwrap();
            }
        }
        if wall_shape.0 {
            if wall_shape.3 {
                tiles[0] = *TILESET.get(&15).unwrap();
            }
            if wall_shape.1 {
                tiles[9] = *TILESET.get(&16).unwrap();
            }
        }
        if wall_shape.2 {
            if wall_shape.3 {
                tiles[70] = *TILESET.get(&17).unwrap();
            }
            if wall_shape.1 {
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
