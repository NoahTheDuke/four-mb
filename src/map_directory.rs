use crate::map::*;
use std::collections::HashMap;

macro_rules! db {
    ($x:literal, $y:literal, $tile:literal) => {
        crate::map::pt($x, $y, $tile)
    };
    ($x:literal, $y:literal, $tile:literal, $vector:literal) => {
        crate::map::ln($x, $y, $tile, $vector)
    };
}

macro_rules! def_maps {(
    $(
        {
            $title:literal, ($top:literal, $right:literal, $bot:literal, $left:literal), $floor:literal;
            $(db $x:literal, $y:literal, $tile:literal $(, $length:literal)?;)*
        },
    )*
) => ({
    let mut rooms = HashMap::new();
    $(
        rooms.insert(
            String::from($title),
            crate::map::Room::new(
                WallShape($top, $right, $bot, $left),
                $floor,
                vec![$(db!($x, $y, $tile $(, $length)*),)*],
            ),
        );
    )*
    rooms
})}

lazy_static! {
    pub static ref ROOMS: HashMap<String, Room> = {
        def_maps![
        {
            "1-3", (true, true, true, true), 4;
            db 0, 6, 17;
            db 0, 7,  2;
            db 1, 5, 34;
            db 1, 6, 20;
            db 1, 7, 17;
            db 2, 2, 44;
            db 2, 6, 34;
            db 3, 1, 44;
            db 4, 3, 35;
            db 6, 1, 44;
            db 7, 2, 44;
            db 7, 6, 34;
            db 8, 5, 34;
            db 8, 6, 19;
            db 8, 7, 18;
        },

        {
            "1-5", (true, true, true, true), 5;
            db 0, 0,  2;
            db 0, 1, 15;
            db 0, 6, 17;
            db 0, 7,  2;
            db 1, 0, 15;
            db 1, 1, 22;
            db 1, 6, 20;
            db 1, 7, 17;
            db 2, 1,  0;
            db 2, 2, 42;
            db 2, 5, 43;
            db 2, 6,  0;
            db 3, 3, 35;
            db 3, 1, 42, 84;
            db 3, 6, 43, 84;
            db 7, 1,  0, 82;
            db 7, 2, 42, 82;
            db 7, 5, 43, 82;
            db 7, 6,  0, 82;
            db 1, 2,  0, 94;
        },

        {
            "1-7", (true, true, true, true), 4;
            db 1, 1, 34;
            db 1, 6, 34;
            db 2, 3,  5;
            db 3, 3, 35;
            db 5, 1, 34;
            db 5, 6, 34;
            db 6, 0, 16;
            db 6, 1, 10;
            db 6, 2, 21;
            db 6, 5, 19;
            db 6, 6, 10;
            db 6, 7, 18;
            db 2, 2,  5, 83;
            db 2, 4,  5, 83;
            db 7, 0,  2, 83;
            db 7, 1,  2, 83;
            db 7, 2,  7, 82;
            db 7, 5,  8, 82;
            db 7, 6,  2, 83;
            db 7, 7,  2, 83;
            db 4, 3,  5, 92;
        },

        {
            "2-3", (true, false, true, true), 4;
            db 0, 2, 26;
            db 1, 1, 34;
            db 1, 3,  5;
            db 1, 5, 51;
            db 1, 6, 34;
            db 2, 3, 32;
            db 4, 2, 32;
            db 4, 7, 29;
            db 5, 2,  5;
            db 5, 3, 32;
            db 6, 2, 32;
            db 6, 3,  5;
            db 7, 3, 32;
            db 1, 2,  5, 83;
            db 2, 5, 44, 82;
            db 3, 3,  5, 82;
            db 4, 5, 51, 86;
            db 4, 6,  5, 86;
            db 7, 2,  5, 83;
            db 8, 3,  5, 82;
        },

        {
            "2-4", (true, false, false, true), 4;
            db 0, 1,  2;
            db 0, 2, 15;
            db 1, 1, 15;
            db 1, 2, 22;
            db 1, 6, 34;
            db 2, 0, 15;
            db 2, 1, 22;
            db 2, 2, 44;
            db 2, 4, 32;
            db 3, 6, 37;
            db 4, 0, 27;
            db 6, 0,  7;
            db 6, 6, 37;
            db 7, 0, 16;
            db 7, 4, 21;
            db 8, 0, 55;
            db 8, 3, 58;
            db 8, 6, 19;
            db 8, 7, 10;
            db 0, 0,  2, 82;
            db 6, 5, 42, 84;
            db 8, 4,  7, 82;
            db 6, 1,  0, 94;
            db 7, 1, 10, 93;
            db 8, 1, 54, 92;
        },

        {
            "2-5", (false, false, true, true), 4;
            db 0, 4, 13;
            db 1, 5, 43;
            db 2, 3, 43;
            db 2, 4, 42;
            db 4, 7,  4;
            db 5, 7,  8;
            db 7, 3, 43;
            db 7, 4, 42;
            db 8, 0, 10;
            db 8, 1, 21;
            db 8, 2, 35;
            db 2, 6, 43, 82;
            db 4, 6,  6, 82;
            db 6, 6, 43, 83;
            db 3, 1, 37, 92;
            db 6, 1, 37, 92;
        },

        {
            "2-6", (true, true, true, true), 4;
            db 0, 0,  2;
            db 0, 1, 15;
            db 0, 6, 17;
            db 0, 7,  2;
            db 1, 0, 15;
            db 1, 1, 22;
            db 1, 2,  6;
            db 1, 5,  6;
            db 1, 6, 20;
            db 1, 7, 17;
            db 4, 0,  4;
            db 5, 0,  7;
            db 8, 0, 16;
            db 8, 1, 21;
            db 8, 2,  6;
            db 8, 5,  6;
            db 8, 6, 19;
            db 8, 7, 18;
            db 2, 1,  6, 82;
            db 2, 6,  6, 82;
            db 6, 1,  6, 82;
            db 6, 6,  6, 82;
            db 3, 3, 37, 92;
            db 6, 3, 37, 92;
        },

        {
            "2-7", (true, true, true, true), 4;
            db 0, 0,  2;
            db 0, 1, 15;
            db 0, 3,  4;
            db 0, 4,  9;
            db 0, 6, 17;
            db 0, 7,  2;
            db 1, 0, 15;
            db 1, 1, 22;
            db 1, 2, 42;
            db 1, 5, 43;
            db 1, 6, 20;
            db 1, 7, 17;
            db 2, 3,  5;
            db 8, 0, 16;
            db 8, 1, 21;
            db 8, 2, 42;
            db 8, 5, 43;
            db 8, 6, 19;
            db 8, 7, 18;
            db 2, 1, 42, 86;
            db 2, 6, 43, 86;
        },

        {
            "3-3", (true, true, true, false), 4;
            db 2, 3, 32;
            db 3, 2, 32;
            db 3, 5, 32;
            db 4, 4, 32;
            db 5, 3, 32;
            db 6, 2, 32;
            db 6, 4,  5;
            db 6, 5, 32;
            db 7, 4, 32;
            db 8, 1, 34;
            db 8, 6, 34;
            db 0, 2,  5, 83;
            db 0, 3,  5, 82;
            db 0, 5, 51, 83;
            db 0, 6,  5, 88;
            db 3, 3,  5, 82;
            db 4, 2,  5, 82;
            db 6, 3,  5, 82;
            db 7, 2,  5, 92;
            db 7, 5,  5, 92;
        },

        {
            "3-4", (false, true, false, false), 4;
            db 0, 0, 51;
            db 0, 3, 53;
            db 0, 6,  8;
            db 0, 7,  2;
            db 1, 2, 50;
            db 1, 3, 57;
            db 1, 6, 20;
            db 1, 7,  9;
            db 2, 3, 15;
            db 2, 4, 22;
            db 3, 4,  0;
            db 4, 1, 35;
            db 4, 4, 42;
            db 5, 4,  0;
            db 6, 3, 16;
            db 6, 4, 21;
            db 7, 2, 49;
            db 7, 3, 58;
            db 0, 4,  7, 82;
            db 0, 5, 42, 84;
            db 1, 0, 34, 82;
            db 2, 2, 53, 85;
            db 3, 0, 51, 83;
            db 3, 3,  7, 83;
            db 5, 5, 42, 84;
            db 6, 0, 34, 82;
            db 7, 4,  7, 82;
            db 8, 0, 51, 82;
            db 8, 3, 53, 82;
        },

        {
            "3-5", (false, true, true, false), 4;
            db 0, 0,  2;
            db 0, 1,  7;
            db 1, 0,  9;
            db 1, 1, 22;
            db 3, 2,  5;
            db 4, 2, 35;
            db 4, 5, 32;
            db 7, 2, 32;
            db 7, 5, 32;
            db 3, 1,  5, 83;
            db 3, 3,  5, 83;
            db 5, 2,  5, 92;
        },

        {
            "3-6", (true, true, true, true), 4;
            db 0, 3,  4;
            db 0, 4,  9;
            db 1, 6, 34;
            db 3, 2, 43;
            db 3, 3,  0;
            db 3, 4, 42;
            db 3, 7, 20;
            db 5, 3, 59;
            db 6, 2, 43;
            db 6, 3,  0;
            db 6, 4, 42;
            db 6, 7, 19;
            db 8, 2, 35;
            db 8, 6, 34;
            db 4, 4, 44, 82;
            db 4, 7,  4, 82;
        },

        {
            "3-7", (true, true, true, true), 4;
            db 0, 2, 22;
            db 0, 5, 20;
            db 1, 1, 34;
            db 1, 6, 34;
            db 3, 0, 22;
            db 3, 2, 37;
            db 3, 7, 20;
            db 6, 0, 21;
            db 6, 7, 19;
            db 8, 1, 34;
            db 8, 6, 34;
            db 2, 1,  5, 86;
            db 2, 6,  5, 86;
            db 4, 0,  4, 82;
            db 4, 7,  5, 82;
            db 6, 2, 37, 82;
            db 0, 3,  4, 92;
            db 1, 2,  5, 94;
            db 2, 2, 37, 93;
            db 7, 3, 37, 92;
            db 8, 2,  5, 94;
        },

        {
            "4-4", (true, true, true, true), 4;
            db 0, 4, 15;
            db 1, 4,  7;
            db 2, 4, 38;
            db 2, 5, 33;
            db 3, 0, 56;
            db 3, 3, 57;
            db 3, 4,  7;
            db 4, 0, 15;
            db 4, 4, 22;
            db 5, 4, 43;
            db 5, 5,  0;
            db 5, 6, 42;
            db 6, 7, 20;
            db 7, 7,  4;
            db 8, 0, 16;
            db 8, 1, 21;
            db 8, 7, 19;
            db 0, 0, 51, 83;
            db 0, 3, 53, 82;
            db 0, 1,  4, 92;
            db 3, 1, 52, 92;
            db 4, 1,  9, 93;
        },

        {
            "4-5", (true, true, true, true), 4;
            db 0, 1, 22;
            db 0, 2,  4;
            db 0, 6, 20;
            db 4, 7,  8;
            db 5, 7, 25;
            db 6, 0, 22;
            db 7, 0,  4;
            db 8, 0, 21;
            db 0, 3, 32, 84;
            db 3, 2, 32, 86;
            db 0, 4,  4, 92;
        },

        {
            "4-6", (true, true, true, true), 4;
            db 0, 2, 22;
            db 0, 5, 20;
            db 1, 1,  0;
            db 1, 2, 42;
            db 1, 5, 43;
            db 1, 6,  0;
            db 3, 1, 34;
            db 3, 5, 37;
            db 4, 0,  7;
            db 5, 0, 23;
            db 6, 1, 34;
            db 6, 5, 37;
            db 8, 2, 35;
            db 3, 2,  5, 84;
            db 3, 3,  5, 84;
            db 3, 4,  5, 84;
            db 0, 3,  4, 92;
            db 4, 1,  5, 95;
            db 5, 1,  5, 95;
        },

        {
            "5-4", (true, true, true, true), 5;
            db 0, 0,  2;
            db 0, 1, 15;
            db 0, 6, 17;
            db 0, 7,  2;
            db 1, 0, 15;
            db 1, 1, 22;
            db 1, 2, 34;
            db 1, 6, 20;
            db 1, 7, 17;
            db 3, 7, 20;
            db 6, 7, 19;
            db 8, 0, 16;
            db 8, 1, 21;
            db 8, 2, 35;
            db 8, 6, 19;
            db 8, 7, 18;
            db 4, 7,  4, 82;
        },

        {
            "5-5", (true, true, true, true), 4;
            db 0, 4, 30;
            db 3, 0, 22;
            db 3, 3, 19;
            db 3, 4, 21;
            db 4, 3,  8;
            db 4, 4,  7;
            db 5, 3, 20;
            db 5, 4, 22;
            db 6, 0, 21;
            db 7, 2, 43;
            db 7, 5, 42;
            db 8, 2, 44;
            db 8, 5, 44;
            db 4, 0,  4, 82;
            db 7, 3,  0, 92;
        },

        {
            "6-2", (true, true, true, true), 4;
            db 1, 1, 34;
            db 1, 6, 34;
            db 2, 0, 16;
            db 2, 4, 21;
            db 3, 0, 21;
            db 3, 1, 37;
            db 3, 3, 37;
            db 3, 4,  7;
            db 5, 7, 25;
            db 6, 0, 22;
            db 6, 1, 37;
            db 6, 3, 37;
            db 6, 4,  7;
            db 7, 0, 15;
            db 7, 4, 22;
            db 8, 1, 34;
            db 8, 6, 34;
            db 3, 2,  5, 84;
            db 4, 4, 38, 82;
            db 4, 5,  5, 82;
            db 4, 6,  5, 82;
            db 2, 1, 10, 93;
            db 4, 1,  5, 93;
            db 5, 1,  5, 93;
            db 7, 1,  9, 93;
        },

        {
            "6-3", (true, true, true, true), 5;
            db 1, 1, 43;
            db 1, 6, 42;
            db 2, 1, 44;
            db 2, 6, 44;
            db 5, 0,  4;
            db 5, 7,  4;
            db 7, 1, 44;
            db 7, 6, 44;
            db 8, 1, 43;
            db 8, 6, 42;
            db 1, 2,  0, 94;
            db 8, 2,  0, 94;
        },

        {
            "6-4", (true, true, true, true), 4;
            db 3, 3, 34;
            db 5, 0, 31;
            db 5, 7,  4;
            db 6, 3, 34;
            db 3, 4, 37, 84;
        },

        {
            "6-5", (true, true, true, true), 5;
            db 0, 3, 62;
            db 0, 4,  4;
            db 0, 5, 62;
            db 4, 0, 60;
            db 5, 0,  4;
            db 6, 0, 60;
        },
        ]
    };
}
