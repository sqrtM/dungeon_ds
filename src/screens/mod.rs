use crate::board::{Color, Tile};
use crate::entities::{Entities, Entity};
use ctru::console::Console;
use std::iter;

pub struct Screens<'screen> {
    pub(crate) top: Console<'screen>,
    pub(crate) bottom: Console<'screen>,
}

impl<'screen> Screens<'screen> {
    pub fn new(top: Console<'screen>, bottom: Console<'screen>) -> Self {
        Self { top, bottom }
    }

    pub fn draw_selected_tile(&self, coords: (usize, usize), tile: &Tile) {
        self.top.select();

        tile.display_large();
        tile.display_lower_description();
        tile.display_side_description(coords);
    }

    pub fn clear_top(&self) {
        self.top.select();
        for _ in 0..30 {
            println!("                                                    ");
        }
    }

    pub fn display_stats(&self) {
        self.top.select();

        println!(
            "\x1B[1;1H{} the {} :: Level {} {}",
            "NAME", "TITLE", 25, "RACE"
        );
        println!("\x1B[3;1H--------------------------------------------------");

        println!("\x1B[5;1HCON: {}", 15);
        println!("\x1B[5;20HMAD: {}", 39);
        println!("\x1B[6;1HDEX: {}", 8);
        println!("\x1B[6;20HINT: {}", 39);

        println!("\x1B[8;1H--------------------------------------------------");

        println!("\x1B[10;1Hstr: {}", 8);
        println!("\x1B[10;20Hdef: {}", 8);
        println!("\x1B[11;1Hspd: {}", 8);
        println!("\x1B[11;20Heva: {}", 8);
        println!("\x1B[12;1Hetc: {}", 8);
        println!("\x1B[12;20Hetc: {}", 8);
        println!("\x1B[13;1Hetc: {}", 8);
        println!("\x1B[13;20Hetc: {}", 8);
        println!("\x1B[14;1Hetc: {}", 8);
        println!("\x1B[14;20Hetc: {}", 8);
        println!("\x1B[15;1Hetc: {}", 8);
        println!("\x1B[15;20Hetc: {}", 8);

        println!("\x1B[17;1H--------------------------------------------------");

        print!("\x1B[19;1HHEALTH:");
        self.print_colored_bar(43, 55, "health");
        print!("\x1B[20;1HALPHA: ");
        self.print_colored_bar(50, 50, "alpha");
        print!("\x1B[21;1HBETA:  ");
        self.print_colored_bar(13, 50, "beta");
        print!("\x1B[22;1HGAMMA: ");
        self.print_colored_bar(2, 50, "gamma");
        print!("\x1B[23;1HHUNGER:");
        self.print_colored_bar(21, 50, "hunger");
        print!("\x1B[24;1HSANITY:");
        self.print_colored_bar(30, 50, "sanity");
        print!("\x1B[25;1HRAGE:  ");
        self.print_colored_bar(49, 50, "");

        println!("\x1B[27;1H--------------------------------------------------");
    }

    fn print_colored_bar(&self, current: u32, max: u32, color_scheme: &str) {
        self.top.select();

        // Calculate percentage of current value relative to max
        let percentage = (current as f64 / max as f64 * 100.0) as u32;

        // Choose ANSI color based on the color scheme and percentage
        let color_code = match color_scheme {
            "health" => match percentage {
                0..=25 => 31,   // Red
                26..=50 => 33,  // Yellow
                51..=75 => 33,  // Yellow
                76..=100 => 32, // Green
                _ => 0,         // Default color
            },
            "alpha" => match percentage {
                0..=25 => 34,   // Blue
                26..=50 => 36,  // Cyan
                51..=75 => 36,  // Cyan
                76..=100 => 35, // Purple
                _ => 0,         // Default color
            },
            "beta" => match percentage {
                0..=25 => 32,   // Green
                26..=50 => 32,  // Green
                51..=75 => 32,  // Green
                76..=100 => 33, // Yellow
                _ => 0,         // Default color
            },
            "gamma" => match percentage {
                0..=25 => 34,   // Blue
                26..=50 => 35,  // Purple
                51..=75 => 35,  // Purple
                76..=100 => 35, // Purple
                _ => 0,         // Default color
            },
            "hunger" => match percentage {
                0..=25 => 33,   // Yellow
                26..=50 => 35,  // Purple
                51..=75 => 35,  // Purple
                76..=100 => 33, // Yellow
                _ => 0,         // Default color
            },
            "sanity" => match percentage {
                0..=25 => 33,   // Yellow
                26..=50 => 35,  // Purple
                51..=75 => 35,  // Purple
                76..=100 => 33, // Yellow
                _ => 0,         // Default color
            },
            _ => match percentage {
                0..=25 => 35,   // Purple
                26..=50 => 35,  // Purple
                51..=75 => 35,  // Purple
                76..=100 => 33, // Yellow
                _ => 0,         // Default color
            },
        };

        // Calculate the number of characters to represent the filled part of the bar
        let filled_width = ((current as f64 / max as f64) * 36f64).round() as usize;

        // Print colored bar
        print!("[");
        print!("\x1B[{};1m", color_code);
        for _ in 0..filled_width {
            print!("=");
        }
        print!("\x1B[0m"); // Reset color to default
        for _ in filled_width..36 {
            print!("-");
        }
        println!("] {}%", percentage);
    }

    pub fn redraw_map(&self, visible_map: &[Vec<Tile>], player: &Entity, entities: &Entities) {
        let entity_locations: Vec<(usize, usize)> = iter::once(player.location)
            .chain(entities.iter().map(|e| e.location))
            .collect::<Vec<(usize, usize)>>();

        self.bottom.select();
        visible_map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, tile)| {
                let char_to_draw = if (j, i) == player.location { '@' } else { '#' };
                if (j, i) == player.location {
                    print!("{}", Color::Cyan.ansi_code())
                } else {
                    print!("{}", Color::White.ansi_code())
                };
                print!("\x1B[{};{}H", j, i);
                print!("{}", char_to_draw);
                print!("{}", Color::AnsiReset.ansi_code())
            });
            println!();
        })
    }

    pub fn get_touched_tile<'a>(
        &'a self,
        touch_coords: (u16, u16),
        visible_map: &'a [Vec<Tile>],
    ) -> Option<((usize, usize), &Tile)> {
        self.top.select();

        let tile_coords = ((touch_coords.1 / 8) as usize, (touch_coords.0 / 8) as usize);

        visible_map
            .get(tile_coords.0)
            .and_then(move |row| row.get(tile_coords.1).map(|tile| (tile_coords, tile)))
    }
}
