pub mod map;
pub mod tile;
pub mod tile_type;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Eq, Hash, PartialEq)]
pub struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[derive(Copy, Clone)]
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

pub type GameMap = Box<Vec<Vec<Tile>>>;
