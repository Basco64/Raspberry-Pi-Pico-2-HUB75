use crate::colors;
use crate::hub75::{Hub75, Outputs};
use crate::pacman::Pacman;
use crate::text::draw_line_centered;

const LINE1_Y: i32 = 1;
const LINE2_Y: i32 = 9;
const PACMAN_CENTER_Y: i32 = 23;

pub struct IdleFallback {
    pacman: Pacman,
}

impl IdleFallback {
    pub fn new() -> Self {
        Self {
            pacman: Pacman::new(-8, PACMAN_CENTER_Y),
        }
    }

    pub fn step_us(&self) -> u64 {
        self.pacman.step_us()
    }
}

impl<PINS: Outputs> crate::animation::Animation<PINS> for IdleFallback {
    fn tick(&mut self, display: &mut Hub75<PINS>) {
        self.pacman.update();

        display.clear();

        draw_line_centered(display, "CONNECTEZ", LINE1_Y, colors::WHITE);
        draw_line_centered(display, "L'APPLICATION", LINE2_Y, colors::WHITE);

        self.pacman.draw(display);
    }
}
