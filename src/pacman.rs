use embedded_graphics::{drawable::Pixel, pixelcolor::Rgb565, prelude::*};

use crate::hub75::{Hub75, Outputs};

const SIZE: usize = 14;

const PACMAN_CLOSED: [[u8; SIZE]; SIZE] = [
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
];

const PACMAN_OPEN: [[u8; SIZE]; SIZE] = [
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
];

pub struct Pacman {
    pub x: i32,
    pub y: i32,
    mouth_open: bool,
}

impl Pacman {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            mouth_open: true,
        }
    }

    // MS : 100 ms
    pub fn step_us(&self) -> u64 {
        100_000
    }

    pub fn update(&mut self) {
        self.x += 1;
        self.mouth_open = !self.mouth_open;

        if self.x - (SIZE as i32 / 2) > 64 {
            self.x = -(SIZE as i32 / 2);
        }
    }

    pub fn draw<P>(&self, display: &mut P)
    where
        P: Drawing<Rgb565>,
    {
        let yellow = Rgb565::from((255u8, 220u8, 0u8));

        let sprite = if self.mouth_open {
            &PACMAN_OPEN
        } else {
            &PACMAN_CLOSED
        };

        let origin_x = self.x - SIZE as i32 / 2;
        let origin_y = self.y - SIZE as i32 / 2;

        for sy in 0..SIZE {
            for sx in 0..SIZE {
                if sprite[sy][sx] == 0 {
                    continue;
                }

                let px = origin_x + sx as i32;
                let py = origin_y + sy as i32;

                if px < 0 || px >= 64 || py < 0 || py >= 32 {
                    continue;
                }

                display.draw(core::iter::once(Pixel(
                    UnsignedCoord::new(px as u32, py as u32),
                    yellow,
                )));
            }
        }
    }
}

// ============================================================
// ANIMATION TRAIT
// ============================================================

impl<PINS: Outputs> crate::animation::Animation<PINS> for Pacman {
    fn tick(&mut self, display: &mut Hub75<PINS>) {
        self.update();
        display.clear();
        self.draw(display);
    }
}
