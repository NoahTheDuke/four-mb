use crate::part::Part;
use std::collections::HashMap;
use std::convert::TryInto;

/// The raw bytes of the colors in a full square. Each part is 8 bytes. The follow indices are
/// given for each 16-byte chunk:
///
/// 0: Black
/// 1: Dark grey
/// 2: Light grey
/// 3: White
/// 4: Control character to indicate part should not be rendered
/// 5-7: Unused
pub static COLORS: &'static [u8] = include_bytes!("../assets/colors.2bpp");

/// The raw bytes of the dungeon floor images. Each part is 8 bytes. The follow indices are given
/// for each 16-byte chunk:
///
/// 0-3: Dirt
/// 4-7: Mosaic
/// 8-11: Cracked
pub static FLOORS: &'static [u8] = include_bytes!("../assets/floor-base.2bpp");

/// The raw bytes of the dungeon wall images. Each part is 8 bytes. The follow indices are given
/// for each 16-byte chunk:
///
/// 0-1: Top wall
/// 2-3: Bottom wall
/// 4-5: Left wall
/// 6-7: Right wall
/// 8-9: Top bombable wall
/// 10-11: Bottom bombable wall
/// 12-13: Left bombable wall
/// 14-15: Right bombable wall
/// 16-19: Top Left/Top Right/Bottom Left/Bottom Right upper inner corner
/// 20-23: Top Left/Top Right/Bottom Left/Bottom Right lower inner corner
/// 24-27: Top Left/Top Right/Bottom Left/Bottom Right upper outer corner
/// 28-31: Top Left/Top Right/Bottom Left/Bottom Right lower outer corner
pub static WALLS: &'static [u8] = include_bytes!("../assets/walls-base.2bpp");

/// The raw bytes of the door images. Each part is 8 bytes. The following indices are given for
/// each 16-byte chunk:
///
/// 0-3: Top blank door
/// 4-7: Right blank door
/// 8-11: Bottom blank door
/// 12-15: Left blank door
/// 16-17: Top key base parts
/// 18-19: Bottom key base parts
/// 20-21: Left key base parts
/// 22-23: Right key base parts
/// 24-27: Boss door
pub static DOORS: &'static [u8] = include_bytes!("../assets/doors.2bpp");

/// The raw bytes of the dungeon object images. Each part is 8 bytes. The follow indices are given
/// for each 16-byte chunk:
///
/// 0-3: Pyramid block
/// 4-7: Key block
/// 8-11: Fire pit
/// 12-13: Locked chest lid
/// 14-15: Chest base
/// 16-17: Unlocked chest lid
/// 18-21: Gargoyle
/// 22: Top wall torch
/// 23: Right wall torch
/// 24: Left wall torch
/// 25: Bottom wall torch
/// 26: Top stairs
/// 27: Right stairs
/// 28: Left stairs
/// 29: Bottom stairs
/// 30: Top pit overhang
/// 31: Bottom pit overhand
/// 32-33: Top/bottom broken wall
/// 34-35: Left/right broken wall
/// 36: Single part railing
/// 37: Unused
/// 38-41: Button
pub static OBJECTS: &'static [u8] = include_bytes!("../assets/objects.2bpp");

lazy_static! {
    pub static ref PARTS: HashMap<usize, Part> = {
        [COLORS, FLOORS, WALLS, DOORS, OBJECTS]
            .concat()
            .chunks(16)
            .map(|chunk| Part::new(chunk.try_into().unwrap()))
            .enumerate()
            .collect()
    };
}
