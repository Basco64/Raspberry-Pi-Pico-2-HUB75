use crate::hub75::{Hub75, Outputs};
use embedded_graphics::pixelcolor::Rgb565;

use embedded_graphics::{drawable::Pixel, prelude::*, Drawing};

pub mod basque;
pub mod belgium;
pub mod french;
pub mod germany;
pub mod ireland;
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

use basque::basque_flag;
use belgium::belgium_flag;
use french::french_flag;
use germany::germany_flag;
use ireland::ireland_flag;
use italy::italy_flag;
use japan::japan_flag;
use netherlands::netherlands_flag;
use portugal::portugal_flag;
use spain::spain_flag;
use switzerland::switzerland_flag;
use united_kingdom::uk_flag;
use usa::usa_flag;

pub fn dispatch<PINS: Outputs>(cmd: &str, display: &mut Hub75<PINS>) -> bool {
    match cmd {
        "flag_basque" => {
            basque_flag(display);
            true
        }
        "flag_belgium" => {
            belgium_flag(display);
            true
        }
        "flag_french" => {
            french_flag(display);
            true
        }
        "flag_germany" => {
            germany_flag(display);
            true
        }
        "flag_ireland" => {
            ireland_flag(display);
            true
        }
        "flag_italy" => {
            italy_flag(display);
            true
        }
        "flag_japan" => {
            japan_flag(display);
            true
        }
        "flag_netherlands" => {
            netherlands_flag(display);
            true
        }
        "flag_portugal" => {
            portugal_flag(display);
            true
        }
        "flag_spain" => {
            spain_flag(display);
            true
        }
        "flag_switzerland" => {
            switzerland_flag(display);
            true
        }
        "flag_uk" => {
            uk_flag(display);
            true
        }
        "flag_usa" => {
            usa_flag(display);
            true
        }
        _ => false,
    }
}
