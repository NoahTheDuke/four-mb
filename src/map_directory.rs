use crate::map::*;
use std::collections::HashMap;

lazy_static! {
    pub static ref ROOMS: HashMap<String, Room> = {
        let instructions: Vec<Inst> = vec![
            point(3, 0, 22),
            line(4, 0, 4, 0x82),
            point(6, 0, 21),
            point(0, 2, 22),
            line(0, 3, 4, 0xc2),
            point(0, 5, 20),
            line(1, 2, 5, 0xc4),
            line(8, 2, 5, 0xc4),
            line(2, 1, 5, 0x86),
            point(6, 2, 5),
            point(7, 6, 5),
            line(2, 2, 37, 0xc3),
            line(7, 2, 37, 0xc3),
            point(3, 2, 37),
            point(6, 2, 37),
            point(1, 1, 34),
            point(8, 1, 34),
            point(1, 6, 34),
            point(8, 6, 34),
        ];
        let mut rooms = HashMap::new();
        rooms.insert(
            String::from("example"),
            Room::new(
                WallShape {
                    ..Default::default()
                },
                4,
                instructions,
            ),
        );
        rooms
    };
}
