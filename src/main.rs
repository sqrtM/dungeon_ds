mod board;

use crate::board::map::Create;
use crate::board::{Coord, GameMap, Tile, TileType};
use ctru::prelude::*;
use std::collections::HashMap;

static BOTTOM_SCREEN_DIMENSIONS: (usize, usize) = (40, 30);
static TOP_SCREEN_DIMENSIONS: (usize, usize) = (50, 30);

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let top_screen = Console::new(gfx.top_screen.borrow_mut());
    let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());

    let map: GameMap = GameMap::create();

    top_screen.select();
    println!("This is the top screen! There are some tiles in the map!!",);

    bottom_screen.select();
    println!("\x1b[14;00HThis is the bottom screen.");
    println!("There's not as much space down here, but that's okay.");

    top_screen.select();
    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_numbers() {
        assert_eq!(2, 2);
    }
}
