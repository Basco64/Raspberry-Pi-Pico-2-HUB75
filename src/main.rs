#![no_std]
#![no_main]

use panic_halt as _;

mod colors;
mod flags;
mod fm6126a;
mod hub75;
mod init;
mod tests;
mod usb_serial;

use rp235x_hal as hal;

use colors::PALETTE;
use tests::Rng;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[hal::entry]
fn main() -> ! {
    let (mut display, mut timer, usb_bus) = init::init();

    usb_serial::init_usb(usb_bus);

    let mut line_buf: heapless::String<64> = heapless::String::new();
    let mut rng = Rng::new(0xDEADBEEF);

    // Mode par défaut au démarrage : bruit aléatoire en boucle
    let mut random_loop_active = true;
    tests::random_frame(&mut display, &mut rng, &PALETTE);
    let mut last_random_update = timer.get_counter().ticks();

    loop {
        display.output(&mut timer);

        if let Some(line) = usb_serial::poll_line(&mut line_buf) {
            let cmd = line.trim();

            if cmd == "random_loop" {
                random_loop_active = true;
                tests::random_frame(&mut display, &mut rng, &PALETTE);
                last_random_update = timer.get_counter().ticks();
                usb_serial::print("-> random_loop\r\n");
            } else {
                // pour add une catégorie, add juste `|| nouveau_module::dispatch(cmd, &mut display)`
                let handled =
                    tests::dispatch(cmd, &mut display) || flags::dispatch(cmd, &mut display);

                if handled {
                    random_loop_active = false;
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

        if random_loop_active {
            let now = timer.get_counter().ticks();
            if now.wrapping_sub(last_random_update) >= 5_000 {
                tests::random_frame(&mut display, &mut rng, &PALETTE);
                last_random_update = now;
            }
        }
    }
}
