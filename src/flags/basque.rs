use crate::colors::*;
use crate::hub75::{Hub75, Outputs};

use super::set_pixel;

pub fn basque_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let mut color = RED;

            let expected1 = x / 2;
            let diag1 = y >= expected1.saturating_sub(2) && y <= expected1 + 2;

            let expected2 = 31u32.saturating_sub(x / 2);
            let diag2 = y >= expected2.saturating_sub(2) && y <= expected2 + 2;

            if diag1 || diag2 {
                color = GREEN;
            }

            if y >= 14 && y <= 17 {
                color = WHITE;
            }

            if x >= 30 && x <= 33 {
                color = WHITE;
            }

            set_pixel(display, x, y, color);
        }
    }
}
