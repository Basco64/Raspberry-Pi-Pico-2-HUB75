#![no_std]
#![no_main]

use panic_halt as _;

mod animation;
mod colors;
mod fm6126a;
mod hub75;
mod idle;
mod image;
mod init;
mod pacman;
mod storage;
mod text;
mod usb_serial;

use rp235x_hal as hal;

use animation::ActiveAnimation;
use idle::IdleFallback;
use text::ScrollingText;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[hal::entry]
fn main() -> ! {
    let (mut display, mut timer, usb_bus) = init::init();

    usb_serial::init_usb(usb_bus);

    let mut line_buf: heapless::String<64> = heapless::String::new();
    let mut receiving_image: Option<image::ImageReceiver> = None;

    // Au démarrage : reprend le dernier texte sauvegardé en flash, sinon
    // Pacman + message "branchez au PC" en boucle
    let mut saved_text: heapless::String<64> = heapless::String::new();
    let mut active = if storage::load_text(&mut saved_text) {
        ActiveAnimation::Text(ScrollingText::new(&saved_text, colors::WHITE))
    } else {
        ActiveAnimation::Idle(IdleFallback::new())
    };

    active.tick(&mut display);
    let mut last_step = timer.get_counter().ticks();

    loop {
        display.output(&mut timer);

        if let Some(recv) = receiving_image.as_mut() {
            if recv.poll() {
                recv.draw(&mut display);
                active = ActiveAnimation::None;
                receiving_image = None;
                usb_serial::print("-> image reçue\r\n");
            }
        }

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
            } else if cmd == "img" {
                receiving_image = Some(image::ImageReceiver::new());
                usb_serial::print("-> réception image...\r\n");
            } else if cmd == "idle" {
                active = ActiveAnimation::Idle(IdleFallback::new());
                active.tick(&mut display);
                last_step = timer.get_counter().ticks();
                usb_serial::print("-> idle\r\n");
            } else {
                usb_serial::print("commande inconnue: ");
                usb_serial::print(cmd);
                usb_serial::print("\r\n");
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
