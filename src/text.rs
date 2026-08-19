use embedded_graphics::{drawable::Pixel, pixelcolor::Rgb565, prelude::*, Drawing};

use crate::hub75::{Hub75, Outputs};

const GLYPH_W: i32 = 5;
const GLYPH_H: i32 = 7;
const GLYPH_SPACING: i32 = 1;
const ADVANCE: i32 = GLYPH_W + GLYPH_SPACING;

/// Police 5x7 minimale : A-Z, 0-9, espace. Chaque ligne est un u8 dont
/// les 5 bits de poids faible (bit4=colonne gauche ... bit0=colonne
/// droite) représentent les pixels allumés. Pour ajouter un caractère,
/// ajoutez une entrée dans ce match en suivant le même format.
const fn glyph(c: char) -> [u8; 7] {
    match c {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10001, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        'X' => [
            0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b01010, 0b10001,
        ],
        'Y' => [
            0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        '0' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
        ],
        '6' => [
            0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100,
        ],
        _ => [0, 0, 0, 0, 0, 0, 0],
    }
}

fn draw_glyph<PINS: Outputs>(display: &mut Hub75<PINS>, c: char, ox: i32, oy: i32, color: Rgb565) {
    let rows = glyph(c.to_ascii_uppercase());
    for (ry, row_bits) in rows.iter().enumerate() {
        for cx in 0..GLYPH_W {
            let bit = (row_bits >> (GLYPH_W - 1 - cx)) & 1;
            if bit == 0 {
                continue;
            }
            let x = ox + cx;
            let y = oy + ry as i32;
            if x >= 0 && x < 64 && y >= 0 && y < 32 {
                display.draw(core::iter::once(Pixel(
                    UnsignedCoord::new(x as u32, y as u32),
                    color,
                )));
            }
        }
    }
}

pub fn draw_line_centered<PINS: Outputs>(
    display: &mut Hub75<PINS>,
    text: &str,
    oy: i32,
    color: Rgb565,
) {
    let count = text.chars().count() as i32;
    if count == 0 {
        return;
    }
    let width = count * ADVANCE - GLYPH_SPACING;
    let ox = (64 - width) / 2;
    let mut cursor = ox;
    for c in text.chars() {
        draw_glyph(display, c, cursor, oy, color);
        cursor += ADVANCE;
    }
}

pub struct ScrollingText {
    text: heapless::String<64>,
    offset: i32,
    color: Rgb565,
}

impl ScrollingText {
    pub fn new(text: &str, color: Rgb565) -> Self {
        let mut s = heapless::String::new();
        let _ = s.push_str(text);
        Self {
            text: s,
            offset: -64,
            color,
        }
    }

    pub fn step_us(&self) -> u64 {
        60_000 // ~60ms par pixel de défilement
    }

    fn total_width(&self) -> i32 {
        self.text.chars().count() as i32 * ADVANCE
    }

    pub fn full_pass_duration_us(&self) -> u64 {
        let steps = self.total_width() + 128;
        (steps.max(1) as u64) * self.step_us()
    }
}

impl<PINS: Outputs> crate::animation::Animation<PINS> for ScrollingText {
    fn tick(&mut self, display: &mut Hub75<PINS>) {
        self.offset += 1;
        if self.offset > self.total_width() + 64 {
            self.offset = -64;
        }

        display.clear();

        let mut cursor = -self.offset;
        let oy = (32 - GLYPH_H) / 2;
        for c in self.text.chars() {
            draw_glyph(display, c, cursor, oy, self.color);
            cursor += ADVANCE;
        }
    }
}

pub struct PhraseRotation {
    phrases: &'static [&'static str],
    color: Rgb565,
    index: usize,
    current: ScrollingText,
    elapsed_us: u64,
}

const PHRASE_DURATION_US: u64 = 5 * 60 * 1_000_000; // 5 minutes

impl PhraseRotation {
    pub fn new(phrases: &'static [&'static str], color: Rgb565) -> Self {
        let first = phrases.first().copied().unwrap_or("");
        Self {
            phrases,
            color,
            index: 0,
            current: ScrollingText::new(first, color),
            elapsed_us: 0,
        }
    }

    pub fn step_us(&self) -> u64 {
        self.current.step_us()
    }
}

impl<PINS: Outputs> crate::animation::Animation<PINS> for PhraseRotation {
    fn tick(&mut self, display: &mut Hub75<PINS>) {
        self.elapsed_us = self.elapsed_us.saturating_add(self.current.step_us());

        if self.elapsed_us >= PHRASE_DURATION_US && !self.phrases.is_empty() {
            self.index = (self.index + 1) % self.phrases.len();
            self.current = ScrollingText::new(self.phrases[self.index], self.color);
            self.elapsed_us = 0;
        }

        crate::animation::Animation::tick(&mut self.current, display);
    }
}
