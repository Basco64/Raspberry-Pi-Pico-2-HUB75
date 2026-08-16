use crate::colors::*;
use crate::hub75::{Hub75, Outputs};

use super::set_pixel;

pub fn switzerland_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let mut color = RED;

            if (x >= 26 && x <= 37) || (y >= 10 && y <= 21) {
                color = WHITE;
            }

            set_pixel(display, x, y, color);
        }
    }
}
