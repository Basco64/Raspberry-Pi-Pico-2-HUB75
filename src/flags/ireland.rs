use crate::colors::*;
use crate::hub75::{Hub75, Outputs};

use super::set_pixel;

pub fn ireland_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let color = if x < 21 {
                GREEN
            } else if x < 43 {
                WHITE
            } else {
                ORANGE
            };

            set_pixel(display, x, y, color);
        }
    }
}
