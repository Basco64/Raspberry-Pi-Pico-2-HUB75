use embedded_graphics::pixelcolor::Rgb565;

/// Luminosité globale : 255 = pleine puissance, ajustez selon vos goûts
pub const BRIGHTNESS: u8 = 140;

/// Convertit du RGB888 (0-255 par canal) vers RGB565, en `const fn`
/// pour pouvoir définir des constantes de couleur calculées à la compilation.
pub const fn rgb565(r: u8, g: u8, b: u8) -> Rgb565 {
    let r = ((r as u16 * BRIGHTNESS as u16) / 255) as u8;
    let g = ((g as u16 * BRIGHTNESS as u16) / 255) as u8;
    let b = ((b as u16 * BRIGHTNESS as u16) / 255) as u8;

    let r5 = (r as u16) >> 3;
    let g6 = (g as u16) >> 2;
    let b5 = (b as u16) >> 3;
    Rgb565((r5 << 11) | (g6 << 5) | b5)
}

// --- Couleurs de base ---<
pub const BLACK: Rgb565 = rgb565(0, 0, 0);
pub const WHITE: Rgb565 = rgb565(255, 255, 255);
pub const RED: Rgb565 = rgb565(255, 0, 0);
pub const GREEN: Rgb565 = rgb565(0, 255, 0);
pub const BLUE: Rgb565 = rgb565(0, 0, 255);

// --- Secondaires ---
pub const YELLOW: Rgb565 = rgb565(255, 255, 0);
pub const CYAN: Rgb565 = rgb565(0, 255, 255);
pub const MAGENTA: Rgb565 = rgb565(255, 0, 255);
pub const ORANGE: Rgb565 = rgb565(255, 165, 0);
pub const PURPLE: Rgb565 = rgb565(128, 0, 128);
pub const PINK: Rgb565 = rgb565(255, 105, 180);

// --- Tons de gris ---
pub const GRAY: Rgb565 = rgb565(128, 128, 128);
pub const LIGHT_GRAY: Rgb565 = rgb565(200, 200, 200);
pub const DARK_GRAY: Rgb565 = rgb565(64, 64, 64);

// --- Autres teintes utiles ---
pub const BROWN: Rgb565 = rgb565(139, 69, 19);
pub const GOLD: Rgb565 = rgb565(255, 215, 0);
pub const SILVER: Rgb565 = rgb565(192, 192, 192);
pub const NAVY: Rgb565 = rgb565(0, 0, 128);
pub const TEAL: Rgb565 = rgb565(0, 128, 128);
pub const LIME: Rgb565 = rgb565(50, 205, 50);
pub const INDIGO: Rgb565 = rgb565(75, 0, 130);
pub const VIOLET: Rgb565 = rgb565(238, 130, 238);
pub const CORAL: Rgb565 = rgb565(255, 127, 80);
pub const TURQUOISE: Rgb565 = rgb565(64, 224, 208);
pub const CRIMSON: Rgb565 = rgb565(220, 20, 60);
pub const SKY_BLUE: Rgb565 = rgb565(135, 206, 235);
pub const FOREST_GREEN: Rgb565 = rgb565(34, 139, 34);
pub const HOT_PINK: Rgb565 = rgb565(255, 105, 180);
pub const AMBER: Rgb565 = rgb565(255, 191, 0);

pub const PALETTE: [Rgb565; 11] = [
    RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA, ORANGE, PURPLE, PINK, NAVY, CORAL,
];
