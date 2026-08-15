use crate::hub75;
use embedded_hal_0_2::digital::v2::OutputPin;

fn fm6126a_write_register<P: hub75::Outputs>(
    pins: &mut P,
    pattern: &[u8; 16],
    latched_pulses: usize,
    width: usize,
) {
    for l in 0..width {
        let bit = pattern[l % 16] != 0;
        if bit {
            pins.r1().set_high().ok();
            pins.g1().set_high().ok();
            pins.b1().set_high().ok();
            pins.r2().set_high().ok();
            pins.g2().set_high().ok();
            pins.b2().set_high().ok();
        } else {
            pins.r1().set_low().ok();
            pins.g1().set_low().ok();
            pins.b1().set_low().ok();
            pins.r2().set_low().ok();
            pins.g2().set_low().ok();
            pins.b2().set_low().ok();
        }
        if l >= width - latched_pulses {
            pins.lat().set_high().ok();
            pins.oe().set_high().ok();
        } else {
            pins.lat().set_low().ok();
            pins.oe().set_low().ok();
        }
        pins.clk().set_high().ok();
        pins.clk().set_low().ok();
    }
    pins.lat().set_low().ok();
    pins.oe().set_low().ok();
}

pub fn fm6126a_init<P: hub75::Outputs>(pins: &mut P) {
    const WIDTH: usize = 64;
    // Registre 1 : 11 impulsions CLK avec LAT maintenu haut à la fin
    let reg1: [u8; 16] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    // Registre 2 : 12 impulsions CLK avec LAT maintenu haut à la fin
    let reg2: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];

    fm6126a_write_register(pins, &reg1, 11, WIDTH);
    fm6126a_write_register(pins, &reg2, 12, WIDTH);
}
