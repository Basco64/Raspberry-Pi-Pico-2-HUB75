use crate::hub75::{Hub75, Outputs};

use embedded_graphics::{drawable::Pixel, pixelcolor::Rgb565, prelude::*, Drawing};

pub mod crab;
pub mod squid;
// ============================================================
// UTILITAIRE COMMUN
// ============================================================

pub fn draw_sprite<PINS: Outputs, const W: usize, const H: usize>(
    display: &mut Hub75<PINS>,
    sprite: &[[u8; W]; H],
    color: Rgb565,
) {
    display.clear();

    let ox = (64 - W as i32) / 2;
    let oy = (32 - H as i32) / 2;

    for (py, row) in sprite.iter().enumerate() {
        for (px, &on) in row.iter().enumerate() {
            if on == 0 {
                continue;
            }

            let x = ox + px as i32;
            let y = oy + py as i32;

            if x >= 0 && x < 64 && y >= 0 && y < 32 {
                display.draw(core::iter::once(Pixel(
                    UnsignedCoord::new(x as u32, y as u32),
                    color,
                )));
            }
        }
    }
}

// ============================================================
// DISPATCH
// ============================================================

pub fn dispatch<PINS: Outputs>(cmd: &str, _display: &mut Hub75<PINS>) -> bool {
    match cmd {
        // "invader_crab" => {
        //     invader_crab::draw(display);
        //     true
        // }
        _ => false,
    }
}
