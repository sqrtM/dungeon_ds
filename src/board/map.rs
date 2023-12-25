use crate::board::{GameMap, Tile};

pub trait Create {
    fn create() -> Self;
}

impl Create for GameMap {
    fn create() -> Self {
        let mut map = vec![];
        for _ in 0..24 {
            map.push(vec![Tile::default(); 32]);
        }
        Box::new(map)
    }
}
