mod board;
mod entities;
mod items;
mod screens;

use crate::board::map::Create;
use crate::board::{Tile, TileMap};
use crate::entities::{Entities, Entity};
use crate::screens::Screens;
use ctru::prelude::*;

fn main() {
    let apt = Apt::new().expect("Can't connect to Applet service!");
    let mut hid = Hid::new().expect("Can't connect to HID service!");
    let gfx = Gfx::new().expect("Can't connect to Graphics service!");

    let screens = Screens::new(
        Console::new(gfx.top_screen.borrow_mut()),
        Console::new(gfx.bottom_screen.borrow_mut()),
    );

    let map: TileMap = TileMap::create();
    let mut player = Entity::init_player();
    let entities: Entities = Box::new(vec![]);

    let mut old_touch_position = (0, 0);

    let mut input_lag = 0;

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();

        let pressed_keys = hid.keys_down();
        if !pressed_keys.is_empty() && input_lag <= 0 {
            if pressed_keys.contains(KeyPad::START) {
                break;
            }
            player.check_for_movement_input(&pressed_keys);
            screens.redraw_map(&map, &player, &entities);
            input_lag = 30;
        }

        let touch = hid.touch_position();
        if touch != old_touch_position && touch != (0, 0) {
            old_touch_position = touch;
            match screens.get_touched_tile(touch, &map) {
                Some(touched_tile) => {
                    screens.redraw_selected_tile(touched_tile.0, &touched_tile.1);
                }
                None => {}
            };
        }

        input_lag -= 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(2, 2);
    }
}
