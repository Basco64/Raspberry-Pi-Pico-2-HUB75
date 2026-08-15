use crate::colors::*;
use crate::hub75::{Hub75, Outputs};

use embedded_graphics::{drawable::Pixel, pixelcolor::Rgb565, prelude::*, Drawing};

use rp235x_hal as hal;

// ============================================================
// TEST ROUGE
// ============================================================

pub fn red_test<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            display.draw(core::iter::once(Pixel(UnsignedCoord::new(x, y), RED)));
        }
    }
}

// ============================================================
// TEST RANDOM
// ============================================================

pub struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self(if seed == 0 { 0xDEADBEEF } else { seed })
    }
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }

    pub fn next_color(&mut self, palette: &[Rgb565]) -> Rgb565 {
        let idx = (self.next() as usize) % palette.len();
        palette[idx]
    }
}

pub fn random_test<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    let mut rng = Rng(0xDEADBEEF);

    let palette = [
        RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA, ORANGE, PURPLE, PINK, WHITE,
    ];

    for y in 0u32..32 {
        for x in 0u32..64 {
            let color = rng.next_color(&palette);

            display.draw(core::iter::once(Pixel(UnsignedCoord::new(x, y), color)));
        }
    }
}

// ============================================================
// TEST DEGRADE
// ============================================================

pub fn gradient_test<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let r = (x * 255 / 63) as u8;
            let g = (y * 255 / 31) as u8;
            let b = 128u8;

            let color = rgb565(r, g, b);

            display.draw(core::iter::once(Pixel(UnsignedCoord::new(x, y), color)));
        }
    }
}

// ============================================================
// FRAME RANDOM
// ============================================================

pub fn random_frame<PINS: Outputs>(display: &mut Hub75<PINS>, rng: &mut Rng, palette: &[Rgb565]) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let color = rng.next_color(palette);

            display.draw(core::iter::once(Pixel(UnsignedCoord::new(x, y), color)));
        }
    }
}

// ============================================================
// RANDOM QUI CHANGE TOUTES LES 5 SECONDES
// ============================================================

pub fn random_loop_test<PINS: Outputs>(
    display: &mut Hub75<PINS>,
    timer: &mut hal::Timer<hal::timer::CopyableTimer0>,
) -> ! {
    let mut rng = Rng(0xDEADBEEF);

    random_frame(display, &mut rng, &PALETTE);

    let mut last_update = timer.get_counter().ticks();

    loop {
        display.output(timer);

        let now = timer.get_counter().ticks();

        if now.wrapping_sub(last_update) >= 5_000 {
            random_frame(display, &mut rng, &PALETTE);

            last_update = now;
        }
    }
}

// ============================================================
// DRAPEAU BASQUE
// ============================================================

pub fn basque_flag_test<PINS: Outputs>(display: &mut Hub75<PINS>) {
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
