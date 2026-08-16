use crate::colors::*;
use crate::hub75::{Hub75, Outputs};

use super::set_pixel;

pub fn uk_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let mut color = BLUE;
            let d1 = ((x as i32) - 2 * (y as i32)).abs();
            let d2 = ((x as i32) - (62 - 2 * (y as i32))).abs();

            if d1 <= 3 || d2 <= 3 {
                color = WHITE;
            }

            if d1 <= 1 || d2 <= 1 {
                color = RED;
            }

            if x >= 27 && x <= 36 {
                color = WHITE;
            }

            if y >= 12 && y <= 19 {
                color = WHITE;
            }

            if x >= 30 && x <= 33 {
                color = RED;
            }

            if y >= 14 && y <= 17 {
                color = RED;
            }

            set_pixel(display, x, y, color);
        }
    }
}
