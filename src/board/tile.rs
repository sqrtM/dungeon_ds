use crate::board::{Tile, TileType};

impl Tile {
    fn new(tt: TileType, elev: f64) -> Self {
        Self {
            tile_type: tt,
            default_char: tt.get_char(),
            elevation: elev,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::WALL,
            default_char: '#',
            elevation: 0.0,
        }
    }
}
