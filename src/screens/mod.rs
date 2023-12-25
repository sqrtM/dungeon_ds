use crate::board::Tile;
use ctru::console::Console;

static TOP_SCREEN_DIMENSIONS: (usize, usize) = (50, 30);
static BOTTOM_SCREEN_DIMENSIONS: (usize, usize) = (40, 30);

pub struct Screens<'screen> {
    pub(crate) top: Console<'screen>,
    pub(crate) bottom: Console<'screen>,
}

impl<'screen> Screens<'screen> {
    pub fn new(top: Console<'screen>, bottom: Console<'screen>) -> Self {
        Self { top, bottom }
    }

    pub fn redraw_selected_tile(&self, coords: (usize, usize), tile: &Tile) {
        self.top.select();
        println!(
            "\x1b[5,5HSelected Tile is at: {coords:?} and is of type {:?}",
            tile.tile_type
        );
    }

    pub fn redraw_map(&self, visible_map: &[Vec<Tile>]) {
        self.bottom.select();
        visible_map.iter().for_each(|c| {
            c.iter().for_each(|t| print!("{}", t.default_char));
            println!();
        })
    }

    pub fn get_touched_tile(
        &self,
        touch_coords: (u16, u16),
        visible_map: &[Vec<Tile>],
    ) -> Option<((usize, usize), Tile)> {
        self.top.select();

        let tile_coords = ((touch_coords.1 / 8) as usize, (touch_coords.0 / 8) as usize);

        visible_map
            .get(tile_coords.0)
            .and_then(|row| row.get(tile_coords.1).map(|tile| (tile_coords, *tile)))
    }
}
