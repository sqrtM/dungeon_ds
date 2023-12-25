mod color;
pub mod map;
pub mod tile;
pub mod tile_type;

use crate::items::Item;

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // Reset to default color
    AnsiReset,
}

pub struct Tile {
    pub(crate) tile_type: TileType,
    pub(crate) default_char: char,
    pub(crate) items: Option<Box<Vec<Item>>>,
    pub(crate) elevation: f64,
}

#[derive(Copy, Clone, Debug)]
pub enum TileType {
    WALL,
    FLOOR,
}

pub type TileMap = Box<Vec<Vec<Tile>>>;
