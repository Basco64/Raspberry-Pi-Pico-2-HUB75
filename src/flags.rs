use crate::colors::*;
use crate::hub75::{Hub75, Outputs};
use embedded_graphics::{drawable::Pixel, pixelcolor::Rgb565, prelude::*, Drawing};

// ============================================================
// DRAPEAU BASQUE
// ============================================================

pub fn basque_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let mut color = RED;

            let diag1 = {
                let expected = x / 2;
                y >= expected.saturating_sub(2) && y <= expected + 2
            };

            let diag2 = {
                let expected = 31u32.saturating_sub(x / 2);
                y >= expected.saturating_sub(2) && y <= expected + 2
            };

            if diag1 || diag2 {
                color = GREEN;
            }

            if y >= 14 && y <= 17 {
                color = WHITE;
            }
            if x >= 30 && x <= 33 {
                color = WHITE;
            }

            display.draw(core::iter::once(Pixel(UnsignedCoord::new(x, y), color)));
        }
    }
}
