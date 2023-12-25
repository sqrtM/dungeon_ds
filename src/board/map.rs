use crate::board::{GameMap, Tile};

pub trait Create {
    fn create() -> Self;
}

impl Create for GameMap {
    fn create() -> Self {
        let mut map: GameMap = Default::default();
        for row in map.iter_mut() {
            for cell in row.borrow_mut().iter_mut() {
                *cell = Tile::default();
            }
        }
        map
    }
}
