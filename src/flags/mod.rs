use crate::colors::Rgb565;
use crate::hub75::{Hub75, Outputs};

use embedded_graphics::{drawable::Pixel, prelude::*, Drawing};

pub mod basque;
pub mod belgium;
pub mod french;
pub mod germany;
pub mod italy;
pub mod japan;
pub mod netherlands;
pub mod portugal;
pub mod spain;
pub mod switzerland;
pub mod united_kingdom;
pub mod usa;

pub fn set_pixel<PINS: Outputs>(display: &mut Hub75<PINS>, x: u32, y: u32, color: Rgb565) {
    display.draw(core::iter::once(Pixel(UnsignedCoord::new(x, y), color)));
}
