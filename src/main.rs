mod board;
mod screens;

use crate::board::map::Create;
use crate::board::{GameMap, Tile};
use crate::screens::Screens;
use ctru::prelude::*;

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();

    let screens = Screens::new(
        Console::new(gfx.top_screen.borrow_mut()),
        Console::new(gfx.bottom_screen.borrow_mut()),
    );
    let map: GameMap = GameMap::create();
    let mut selected_tile = ((0, 0), Tile::default());
    let mut old_touch_position = (0, 0);

    screens.top.select();
    println!("\x1b[29;16HPress Start to exit");

    screens.redraw_map(&map);

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        let touch = hid.touch_position();
        if touch != old_touch_position {
            old_touch_position = touch;
            match screens.get_touched_tile(touch, &map) {
                Some(touched_tile) => {
                    screens.redraw_selected_tile(touched_tile.0, &touched_tile.1);
                    selected_tile = touched_tile
                }
                None => {}
            };
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(2, 2);
    }
}
