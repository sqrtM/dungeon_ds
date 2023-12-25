use crate::board::{Tile, TileMap};

pub trait Create {
    fn create() -> Self;
}

impl Create for TileMap {
    fn create() -> Self {
        let mut map = vec![];
        for _ in 0..24 {
            let mut row = vec![];
            for _ in 0..32 {
                row.push(Tile::default());
            }
            map.push(row);
        }
        Box::new(map)
    }
}
