use crate::colors::*;
use crate::hub75::{Hub75, Outputs};

use super::set_pixel;

pub fn usa_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let stripe = y / 2;

            let mut color = if stripe % 2 == 0 { RED } else { WHITE };

            if x < 28 && y < 14 {
                color = BLUE;
                if (x % 7 == 3) && (y % 4 == 1) {
                    color = WHITE;
                }
            }

            set_pixel(display, x, y, color);
        }
    }
}
