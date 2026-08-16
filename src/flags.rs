use crate::colors::*;
use crate::hub75::{Hub75, Outputs};
use embedded_graphics::{drawable::Pixel, pixelcolor::Rgb565, prelude::*, Drawing};

fn set_pixel<PINS: Outputs>(display: &mut Hub75<PINS>, x: u32, y: u32, color: Rgb565) {
    display.draw(core::iter::once(Pixel(UnsignedCoord::new(x, y), color)));
}

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

            set_pixel(display, x, y, color);
        }
    }
}

// ============================================================
// 🇫🇷 FRANCE
// ============================================================

pub fn france_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let color = if x < 21 {
                BLUE
            } else if x < 43 {
                WHITE
            } else {
                RED
            };

            set_pixel(display, x, y, color);
        }
    }
}

// ============================================================
// 🇮🇹 ITALIE
// ============================================================

pub fn italy_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let color = if x < 21 {
                GREEN
            } else if x < 43 {
                WHITE
            } else {
                RED
            };

            set_pixel(display, x, y, color);
        }
    }
}

// ============================================================
// 🇧🇪 BELGIQUE
// ============================================================

pub fn belgium_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let color = if x < 21 {
                BLACK
            } else if x < 43 {
                YELLOW
            } else {
                RED
            };

            set_pixel(display, x, y, color);
        }
    }
}

// ============================================================
// 🇩🇪 ALLEMAGNE
// ============================================================

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

// ============================================================
// 🇪🇸 ESPAGNE
// ============================================================

pub fn spain_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        let color = if y < 7 {
            RED
        } else if y < 25 {
            YELLOW
        } else {
            RED
        };

        for x in 0u32..64 {
            set_pixel(display, x, y, color);
        }
    }
}

// ============================================================
// 🇳🇱 PAYS-BAS
// ============================================================

pub fn netherlands_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        let color = if y < 11 {
            RED
        } else if y < 22 {
            WHITE
        } else {
            BLUE
        };

        for x in 0u32..64 {
            set_pixel(display, x, y, color);
        }
    }
}

// ============================================================
// 🇮🇪 IRLANDE
// ============================================================

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

// ============================================================
// 🇯🇵 JAPON
// ============================================================

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

// ============================================================
// 🇨🇭 SUISSE
// ============================================================

pub fn switzerland_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let mut color = RED;

            // Croix blanche
            if (x >= 26 && x <= 37) || (y >= 10 && y <= 21) {
                color = WHITE;
            }

            set_pixel(display, x, y, color);
        }
    }
}

// ============================================================
// 🇺🇸 ÉTATS-UNIS
// ============================================================

pub fn usa_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            // 13 bandes
            let stripe = y / 2;

            let mut color = if stripe % 2 == 0 { RED } else { WHITE };

            // Canton bleu
            if x < 28 && y < 14 {
                color = BLUE;

                // Étoiles simplifiées
                if (x % 7 == 3) && (y % 4 == 1) {
                    color = WHITE;
                }
            }

            set_pixel(display, x, y, color);
        }
    }
}

// ============================================================
// 🇬🇧 ROYAUME-UNI
// ============================================================

pub fn uk_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let mut color = BLUE;

            // Diagonale rouge/blanche simplifiée
            let d1 = ((x as i32) - 2 * (y as i32)).abs();
            let d2 = ((x as i32) - (62 - 2 * (y as i32))).abs();

            if d1 <= 3 || d2 <= 3 {
                color = WHITE;
            }

            if d1 <= 1 || d2 <= 1 {
                color = RED;
            }

            // Croix centrale blanche
            if x >= 27 && x <= 36 {
                color = WHITE;
            }

            if y >= 12 && y <= 19 {
                color = WHITE;
            }

            // Croix centrale rouge
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

// ============================================================
// 🇵🇹 PORTUGAL
// ============================================================

pub fn portugal_flag<PINS: Outputs>(display: &mut Hub75<PINS>) {
    display.clear();

    for y in 0u32..32 {
        for x in 0u32..64 {
            let mut color = if x < 25 { GREEN } else { RED };

            // Sphère simplifiée
            let dx = x as i32 - 25;
            let dy = y as i32 - 16;

            if dx * dx + dy * dy <= 7 * 7 {
                color = YELLOW;
            }

            set_pixel(display, x, y, color);
        }
    }
}
