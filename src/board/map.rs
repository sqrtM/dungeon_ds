use crate::board::{GameMap, Tile};

pub trait Create {
    fn create() -> Self;
}

impl Create for GameMap {
    fn create() -> Self {
        let mut map = vec![];
        for i in 0..10 {
            map.push(vec![Tile::default(); 10]);
        }
        Box::new(map)
    }
}
