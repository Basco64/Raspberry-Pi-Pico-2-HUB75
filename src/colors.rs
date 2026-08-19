use embedded_graphics::pixelcolor::Rgb565;

/// Luminosité globale : 255 = pleine puissance
pub const BRIGHTNESS: u8 = 210;

pub const fn rgb565(r: u8, g: u8, b: u8) -> Rgb565 {
    let r = ((r as u16 * BRIGHTNESS as u16) / 255) as u8;
    let g = ((g as u16 * BRIGHTNESS as u16) / 255) as u8;
    let b = ((b as u16 * BRIGHTNESS as u16) / 255) as u8;

    let r5 = (r as u16) >> 3;
    let g6 = (g as u16) >> 2;
    let b5 = (b as u16) >> 3;
    Rgb565((r5 << 11) | (g6 << 5) | b5)
}

pub const WHITE: Rgb565 = rgb565(255, 255, 255);
pub const YELLOW: Rgb565 = rgb565(255, 255, 0);
