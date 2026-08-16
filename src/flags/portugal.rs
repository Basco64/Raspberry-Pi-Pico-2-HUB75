use crate::colors::*;
use crate::hub75::{Hub75, Outputs};

use super::set_pixel;

pub fn portugal_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let mut color = if x < 25 { GREEN } else { RED };

            let dx = x as i32 - 25;
            let dy = y as i32 - 16;

            if dx * dx + dy * dy <= 7 * 7 {
                color = YELLOW;
            }

            set_pixel(display, x, y, color);
        }
    }
}
