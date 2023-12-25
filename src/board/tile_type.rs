use crate::board::TileType;

impl TileType {
    pub fn get_char(&self) -> char {
        match self {
            TileType::WALL => '#',
            TileType::FLOOR => '.',
        }
    }
}
