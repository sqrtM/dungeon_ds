pub mod map;
pub mod tile;
pub mod tile_type;

use std::cell::RefCell;

#[derive(Eq, Hash, PartialEq)]
pub struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

pub struct Tile {
    pub(crate) tile_type: TileType,
    pub(crate) default_char: char,
    pub(crate) elevation: f64,
}

#[derive(Copy, Clone)]
pub enum TileType {
    WALL,
    FLOOR,
}

pub type GameMap = [RefCell<[Tile; 10]>; 10];
