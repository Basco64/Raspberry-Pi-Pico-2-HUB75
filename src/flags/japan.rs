use crate::colors::*;
use crate::hub75::{Hub75, Outputs};

use super::set_pixel;

pub fn japan_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    let cx = 32i32;
    let cy = 16i32;
    let radius = 9i32;

    for y in 0u32..32 {
        for x in 0u32..64 {
            let dx = x as i32 - cx;
            let dy = y as i32 - cy;

            let color = if dx * dx + dy * dy <= radius * radius {
                RED
            } else {
                WHITE
            };

            set_pixel(display, x, y, color);
        }
    }
}
