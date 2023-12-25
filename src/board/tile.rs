use crate::board::{Tile, TileType};

impl Tile {
    fn new(tt: TileType, elev: f64) -> Self {
        Self {
            tile_type: tt,
            default_char: tt.get_char(),
            items: None,
            elevation: elev,
        }
    }

    pub fn display_large(&self) {
        match self.tile_type {
            TileType::WALL => {
                print!("\x1B[1;1H");
                println!("    ####    ####    ");
                println!("    ####    ####    ");
                println!("    ####    ####    ");
                println!("    ####    ####    ");
                println!("####################");
                println!("####################");
                println!("####################");
                println!("####################");
                println!("    ####    ####    ");
                println!("    ####    ####    ");
                println!("    ####    ####    ");
                println!("    ####    ####    ");
                println!("####################");
                println!("####################");
                println!("####################");
                println!("####################");
                println!("    ####    ####    ");
                println!("    ####    ####    ");
                println!("    ####    ####    ");
                println!("    ####    ####    ");
            }
            TileType::FLOOR => {
                // ANSI escape codes to position at row 1, column 1
                print!("\x1B[1;1H");
                // Display for Tilde
                // Add your ASCII art for Tilde here
            }
        }
        print!("\x1B[1;1H");
    }

    pub fn display_lower_description(&self) {
        print!("\x1B[24;1H");
        match self.tile_type {
            TileType::WALL => {
                println!("Tile Description for Hash:");
                println!("Hash represents a solid block.");
            }
            TileType::FLOOR => {
                println!("Tile Description for Hash:");
                println!("Hash represents a solid block.");
            }
        }
        print!("\x1B[1;1H");
    }

    pub fn display_side_description(&self, coord: (usize, usize)) {
        match self.tile_type {
            TileType::WALL => {
                println!("\x1B[1;24HA Wall.");
                println!("\x1B[2;24HNot too much going on here.");
            }
            TileType::FLOOR => {
                println!("\x1B[1;24HAnother description:");
                println!("\x1B[2;24HMaybe its contents?");
            }
        }
        println!("\x1B[5;24HLocation: {coord:?}",);
        println!("\x1B[6;24HElevation: {:#?}", self.elevation);
        println!("\x1B[10;24HContains: {:#?}", self.items);
        print!("\x1B[1;1H");
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::WALL,
            default_char: '#',
            items: None,
            elevation: 0.0,
        }
    }
}
