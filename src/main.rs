#![no_std]
#![no_main]

use panic_halt as _;

mod colors;
mod fm6126a;
mod hub75;
mod init;
mod tests;
mod usb_serial;

use rp235x_hal as hal;

use colors::PALETTE;
use hub75::{Hub75, Outputs};
use tests::{basque_flag_test, gradient_test, random_frame, random_test, red_test, Rng};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    RedTest,
    GradientTest,
    RandomTest,
    RandomLoop,
}

fn apply_mode<PINS: Outputs>(display: &mut Hub75<PINS>, mode: Mode, rng: &mut Rng) {
    match mode {
        Mode::RedTest => red_test(display),
        Mode::GradientTest => gradient_test(display),
        Mode::RandomTest => random_test(display),
        Mode::RandomLoop => random_frame(display, rng, &PALETTE),
    }
}

#[hal::entry]
fn main() -> ! {
    let (mut display, mut timer, usb_bus) = init::init();

    usb_serial::init_usb(usb_bus);

    let mut line_buf: heapless::String<64> = heapless::String::new();

    let mut rng = Rng::new(0xDEADBEEF);
    let mut mode = Mode::RandomLoop;

    apply_mode(&mut display, mode, &mut rng);
    let mut last_random_update = timer.get_counter().ticks();

    loop {
        // Le panneau doit être rafraîchi en continu, sinon il clignote/s'éteint.
        // Maintenant sans risque pour l'USB, qui tourne sur interruption.
        display.output(&mut timer);

        if let Some(line) = usb_serial::poll_line(&mut line_buf) {
            let cmd = line.trim();
            let new_mode = match cmd {
                "red_test" => Some(Mode::RedTest),
                "gradient_test" => Some(Mode::GradientTest),
                "random_test" => Some(Mode::RandomTest),
                "random_loop" => Some(Mode::RandomLoop),
                _ => None,
            };

            match new_mode {
                Some(m) => {
                    mode = m;
                    apply_mode(&mut display, mode, &mut rng);
                    last_random_update = timer.get_counter().ticks();
                    usb_serial::print("-> ");
                    usb_serial::print(cmd);
                    usb_serial::print("\r\n");
                }
                None => {
                    usb_serial::print("commande inconnue: ");
                    usb_serial::print(cmd);
                    usb_serial::print(
                        "\r\nattendu: red_test / gradient_test / random_test / random_loop\r\n",
                    );
                }
            }
        }

        if mode == Mode::RandomLoop {
            let now = timer.get_counter().ticks();
            if now.wrapping_sub(last_random_update) >= 5_000 {
                basque_flag_test(&mut display);
                last_random_update = now;
            }
        }
    }
}
