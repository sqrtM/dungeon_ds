use crate::board::Tile;
use ctru::console::Console;

pub struct Screens<'screen> {
    pub(crate) top: Console<'screen>,
    pub(crate) bottom: Console<'screen>,
}

impl<'screen> Screens<'screen> {
    pub fn new(top: Console<'screen>, bottom: Console<'screen>) -> Self {
        Self { top, bottom }
    }

    pub fn redraw_map(&self, content: &[Vec<Tile>]) {
        self.bottom.select();
        content.iter().for_each(|c| {
            c.iter().for_each(|t| print!("{}", t.default_char));
            println!();
        })
    }
}
