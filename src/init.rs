use rp235x_hal as hal;

use crate::fm6126a::fm6126a_init;
use crate::hub75::Hub75;

const XTAL_FREQ_HZ: u32 = 12_000_000;

pub fn init() -> (
    Hub75<(
        hal::gpio::Pin<hal::gpio::bank0::Gpio0, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio1, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio2, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio3, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio4, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio5, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio6, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio7, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio8, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio9, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio11, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio13, hal::gpio::FunctionSioOutput, hal::gpio::PullDown>,
    )>,
    hal::Timer<hal::timer::CopyableTimer0>,
) {
    let mut pac = hal::pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let r1 = pins.gpio0.into_push_pull_output();
    let g1 = pins.gpio1.into_push_pull_output();
    let b1 = pins.gpio2.into_push_pull_output();

    let r2 = pins.gpio3.into_push_pull_output();
    let g2 = pins.gpio4.into_push_pull_output();
    let b2 = pins.gpio5.into_push_pull_output();

    let a = pins.gpio6.into_push_pull_output();
    let b = pins.gpio7.into_push_pull_output();
    let c = pins.gpio8.into_push_pull_output();
    let d = pins.gpio9.into_push_pull_output();

    let clk = pins.gpio11.into_push_pull_output();
    let lat = pins.gpio12.into_push_pull_output();
    let oe = pins.gpio13.into_push_pull_output();

    // E / FM6126A
    let mut e = pins.gpio10.into_push_pull_output();
    use embedded_hal_0_2::digital::v2::OutputPin;
    e.set_low().ok();

    let mut pins = (r1, g1, b1, r2, g2, b2, a, b, c, d, clk, lat, oe);

    fm6126a_init(&mut pins);

    let mut display = Hub75::new(pins, 4);

    display.clear();

    (display, timer)
}
