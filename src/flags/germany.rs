use crate::colors::*;
use crate::hub75::{Hub75, Outputs};

use super::set_pixel;

pub fn germany_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        let color = if y < 11 {
            BLACK
        } else if y < 22 {
            RED
        } else {
            YELLOW
        };

        for x in 0u32..64 {
            set_pixel(display, x, y, color);
        }
    }
}
