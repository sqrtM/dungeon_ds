use crate::entities::Direction::{E, N, S, W};
use ctru::prelude::KeyPad;

pub struct Entity {
    name: &'static str,
    char: char,
    pub(crate) location: (usize, usize),
}

pub type Entities = Box<Vec<Entity>>;

pub enum Direction {
    N,
    S,
    E,
    W,
}

impl Entity {
    pub fn init_player() -> Self {
        Self {
            name: "Player",
            char: '@',
            location: (1, 1),
        }
    }

    pub fn check_for_movement_input(&mut self, key_pad: &KeyPad) {
        if key_pad.contains(KeyPad::DPAD_UP) {
            self._move(N)
        } else if key_pad.contains(KeyPad::DPAD_DOWN) {
            self._move(S)
        } else if key_pad.contains(KeyPad::DPAD_LEFT) {
            self._move(W)
        } else if key_pad.contains(KeyPad::DPAD_RIGHT) {
            self._move(E)
        }
    }

    fn _move(&mut self, dir: Direction) {
        match dir {
            N => self.location.0 -= 1,
            S => self.location.0 += 1,
            E => self.location.1 += 1,
            W => self.location.1 -= 1,
        }
    }
}
