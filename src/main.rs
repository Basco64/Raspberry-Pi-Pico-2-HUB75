#![no_std]
#![no_main]

use panic_halt as _;

mod colors;
mod fm6126a;
mod hub75;
mod init;
mod tests;

use rp235x_hal as hal;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[hal::entry]
fn main() -> ! {
    let (mut display, mut timer) = init::init();

    tests::random_loop_test(&mut display, &mut timer);
}
