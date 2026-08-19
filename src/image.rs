use embedded_graphics::{drawable::Pixel, pixelcolor::Rgb565, prelude::*, Drawing};

use crate::hub75::{Hub75, Outputs};

const W: usize = 64;
const H: usize = 32;
const BYTES: usize = W * H * 2; // RGB565 = 2 octets/pixel

pub struct ImageReceiver {
    buf: [u8; BYTES],
    received: usize,
}

impl ImageReceiver {
    pub fn new() -> Self {
        Self {
            buf: [0; BYTES],
            received: 0,
        }
    }

    pub fn poll(&mut self) -> bool {
        while self.received < BYTES {
            match crate::usb_serial::read_byte() {
                Some(b) => {
                    self.buf[self.received] = b;
                    self.received += 1;
                }
                None => break,
            }
        }
        self.received >= BYTES
    }

    pub fn draw<PINS: Outputs>(&mut self, display: &mut Hub75<PINS>) {
        display.clear();
        for y in 0..H {
            for x in 0..W {
                let idx = (y * W + x) * 2;
                let raw = u16::from_le_bytes([self.buf[idx], self.buf[idx + 1]]);
                let color = Rgb565::from(raw);
                display.draw(core::iter::once(Pixel(
                    UnsignedCoord::new(x as u32, y as u32),
                    color,
                )));
            }
        }
        self.received = 0;
    }
}
