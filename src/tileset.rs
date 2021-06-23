use crate::tile::Tile;
use std::collections::HashMap;

lazy_static! {
    pub static ref TILESET: HashMap<usize, Tile> = {
        [
            // 0 Black tile
            (0, 0, 0, 0),
            // 1 Dark gray tile
            (1, 1, 1, 1),
            // 2 Light gray tile
            (2, 2, 2, 2),
            // 3 White
            (3, 3, 3, 3),
            // 4 Dirt floor
            (8, 9, 10, 11),
            // 5 Mosaic floor
            (12, 13, 14, 15),
            // 6 Cracked floor
            (16, 17, 18, 19),
            // 7 Top wall
            (20, 20, 21, 21),
            // 8 Bottom wall
            (22, 22, 23, 23),
            // 9 Left wall
            (24, 25, 24, 25),
            // 10 Right wall
            (26, 27, 26, 27),
            // 11 Top cracked wall
            (28, 28, 29, 29),
            // 12 Bottom cracked wall
            (30, 30, 31, 31),
            // 13 Left cracked wall
            (32, 33, 32, 33),
            // 14 Right cracked wall
            (34, 35, 34, 35),
            // 15 Top left inside corner
            (36, 20, 24, 40),
            // 16 Top right inside corner
            (20, 37, 41, 27),
            // 17 Bottom left inside corner
            (24, 42, 38, 23),
            // 18 Bottom right inside corner
            (43, 27, 23, 39),
            // 19 Top left outside corner
            (48, 22, 26, 44),
            // 20 Top right outside corner
            (22, 49, 46, 25),
            // 21 Bottom left outside corner
            (26, 46, 50, 21),
            // 22 Bottom right outside corner
            (47, 25, 21, 51),
            // 23 Top door
            (52, 53, 54, 55),
            // 24 Right door
            (56, 57, 58, 59),
            // 25 Bottom door
            (60, 61, 62, 63),
            // 26 Left door
            (64, 65, 66, 67),
            // 27 Top locked door
            (52, 53, 68, 69),
            // 28 Right locked door
            (73, 57, 75, 59),
            // 29 Bottom locked door
            (70, 71, 62, 63),
            // 30 Left locked door
            (64, 72, 66, 74),
            // 31 Boss door
            (76, 77, 78, 79),
            // 32 Block
            (80, 81, 82, 83),
            // 33 Locked block
            (84, 85, 86, 87),
            // 34 Fire pit
            (88, 89, 90, 91),
            // 35 Locked chest
            (92, 93, 94, 95),
            // 36 Opened chest
            (96, 97, 94, 95),
            // 37 Gargoyle
            (98, 99, 100, 101),
            // 38 Top stairs
            (106, 106, 106, 106),
            // 39 Right stairs
            (107, 107, 107, 107),
            // 40 Left stairs
            (108, 108, 108, 108),
            // 41 Bottom stairs
            (109, 109, 109, 109),
            // 42 Top pit
            (0, 0, 110, 110),
            // 43 Bottom pit
            (111, 111, 0, 0),
            // 44 Both pit
            (111, 111, 110, 110),
            // 45 Top/bottom broken door
            (112, 113, 112, 113),
            // 46 Left/right broken door
            (114, 114, 115, 115),
            // 47 Top left railing
            (116, 4, 4, 4),
            // 48 Top right railing
            (4, 116, 4, 4),
            // 49 Bottom left railing
            (4, 4, 116, 4),
            // 50 Bottom right railing
            (4, 4, 4, 116),
            // 51 Top left/right railing
            (116, 116, 4, 4),
            // 52 Top/bottom right railing
            (4, 116, 4, 116),
            // 53 Bottom left/right railing
            (4, 4, 116, 116),
            // 54 Top/bottom left railing
            (116, 4, 116, 4),
            // 55 Top left/right, bottom left railing
            (116, 116, 116, 4),
            // 56 Top left/right, bottom right railing
            (116, 116, 4, 116),
            // 57 Top right, bottom left/right railing
            (4, 116, 116, 116),
            // 58 Top left, bottom left/right railing
            (116, 4, 116, 116),
            // 59 Button
            (118, 119, 120, 121),
            // 60 Top wall torches
            (117, 117, 102, 102),
            // 61 Rigth wall torches
            (103, 117, 103, 117),
            // 62 Bottom wall torches
            (117, 104, 117, 104),
            // 63 Left wall torches
            (105, 105, 117, 117),
            ]
                .iter()
                .enumerate()
                .map(|(idx, parts)| (idx, Tile::from_tuple(idx, parts)))
                .collect()
    };
}
