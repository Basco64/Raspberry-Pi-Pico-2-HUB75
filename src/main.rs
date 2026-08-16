#![no_std]
#![no_main]

use panic_halt as _;

mod animation;
mod colors;
mod flags;
mod fm6126a;
mod hub75;
mod init;
mod named;
mod space_invaders;
mod storage;
mod tests;
mod text;
mod usb_serial;

use rp235x_hal as hal;

use animation::ActiveAnimation;
use named::pacman::Pacman;
use space_invaders::crab::Crab;
use space_invaders::squid::Squid;
use tests::RandomLoop;
use text::{PhraseRotation, ScrollingText};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

// phrases de test
const PHRASES: &[&str] = &["PICO WALL", "BONJOUR", "COUCOU", "PLOP"];
#[hal::entry]
fn main() -> ! {
    let (mut display, mut timer, usb_bus) = init::init();

    usb_serial::init_usb(usb_bus);

    let mut line_buf: heapless::String<64> = heapless::String::new();

    // default
    let mut saved_text: heapless::String<64> = heapless::String::new();
    // default
    // let mut active = ActiveAnimation::Pacman(Pacman::new(-8, 16));
    let mut active = if storage::load_text(&mut saved_text) {
        ActiveAnimation::Text(ScrollingText::new(&saved_text, colors::WHITE))
    } else {
        ActiveAnimation::Phrases(PhraseRotation::new(PHRASES, colors::WHITE))
    };

    active.tick(&mut display);
    let mut last_step = timer.get_counter().ticks();

    loop {
        display.output(&mut timer);

        if let Some(line) = usb_serial::poll_line(&mut line_buf) {
            let cmd = line.trim();

            if let Some(msg) = cmd.strip_prefix("text:") {
                storage::save_text(msg);
                active = ActiveAnimation::Text(ScrollingText::new(msg, colors::WHITE));
                active.tick(&mut display);
                last_step = timer.get_counter().ticks();
                usb_serial::print("-> text: ");
                usb_serial::print(msg);
                usb_serial::print("\r\n");
            } else {
                match cmd {
                    "random_loop" => {
                        active = ActiveAnimation::Random(RandomLoop::new(0xDEADBEEF));
                        active.tick(&mut display);
                        last_step = timer.get_counter().ticks();
                        usb_serial::print("-> random_loop\r\n");
                    }
                    "pacman" => {
                        active = ActiveAnimation::Pacman(Pacman::new(-8, 16));
                        active.tick(&mut display);
                        last_step = timer.get_counter().ticks();
                        usb_serial::print("-> pacman\r\n");
                    }
                    "squid" => {
                        active = ActiveAnimation::Squid(Squid::new(32, 16));
                        active.tick(&mut display);
                        last_step = timer.get_counter().ticks();
                        usb_serial::print("-> squid\r\n");
                    }
                    "crab" => {
                        active = ActiveAnimation::Crab(Crab::new(32, 16));
                        active.tick(&mut display);
                        last_step = timer.get_counter().ticks();
                        usb_serial::print("-> crab\r\n");
                    }
                    _ => {
                        let handled = tests::dispatch(cmd, &mut display)
                            || flags::dispatch(cmd, &mut display)
                            || space_invaders::dispatch(cmd, &mut display);

                        if handled {
                            active = ActiveAnimation::None;
                            usb_serial::print("-> ");
                            usb_serial::print(cmd);
                            usb_serial::print("\r\n");
                        } else {
                            usb_serial::print("commande inconnue: ");
                            usb_serial::print(cmd);
                            usb_serial::print("\r\n");
                        }
                    }
                }
            }
        }

        if let Some(step_us) = active.step_us() {
            let now = timer.get_counter().ticks();
            if now.wrapping_sub(last_step) >= step_us {
                active.tick(&mut display);
                last_step = now;
            }
        }
    }
}
