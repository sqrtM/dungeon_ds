pub mod map;
pub mod tile;
pub mod tile_type;

use crate::items::Item;

#[derive(Eq, Hash, PartialEq)]
pub struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
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
