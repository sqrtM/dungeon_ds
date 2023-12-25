mod board;
mod screens;

use crate::board::map::Create;
use crate::board::GameMap;
use crate::screens::Screens;
use ctru::prelude::*;

static BOTTOM_SCREEN_DIMENSIONS: (usize, usize) = (40, 30);
static TOP_SCREEN_DIMENSIONS: (usize, usize) = (50, 30);

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();

    let screens = Screens::new(
        Console::new(gfx.top_screen.borrow_mut()),
        Console::new(gfx.bottom_screen.borrow_mut()),
    );
    let map: GameMap = GameMap::create();

    screens.top.select();
    println!("This is the top screen! There are some tiles in the map!!",);

    screens.top.select();
    println!("\x1b[29;16HPress Start to exit");

    screens.redraw_map(&map);

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
