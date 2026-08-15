use rp235x_hal as hal;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::SerialPort;

pub struct UsbSerial<'a> {
    device: UsbDevice<'a, hal::usb::UsbBus>,
    serial: SerialPort<'a, hal::usb::UsbBus>,
}

impl<'a> UsbSerial<'a> {
    pub fn new(bus: &'a UsbBusAllocator<hal::usb::UsbBus>) -> Self {
        let serial = SerialPort::new(bus);
        let device = UsbDeviceBuilder::new(bus, UsbVidPid(0x16c0, 0x27dd))
            .strings(&[StringDescriptors::default()
                .manufacturer("pico-wall")
                .product("HUB75 Console")
                .serial_number("0001")])
            .unwrap()
            .max_packet_size_0(64)
            .unwrap()
            .device_class(usbd_serial::USB_CLASS_CDC)
            .build();

        Self { device, serial }
    }

    /// À appeler à chaque tour de boucle principale.
    /// Retourne Some(ligne) uniquement quand un '\n' a été reçu.
    pub fn poll_line(&mut self, buf: &mut heapless::String<64>) -> Option<heapless::String<64>> {
        if !self.device.poll(&mut [&mut self.serial]) {
            return None;
        }

        let mut data = [0u8; 64];
        if let Ok(count) = self.serial.read(&mut data) {
            for &b in &data[..count] {
                match b {
                    b'\r' | b'\n' => {
                        if !buf.is_empty() {
                            let line = buf.clone();
                            buf.clear();
                            let _ = self.serial.write(b"\r\n");
                            return Some(line);
                        }
                    }
                    0x08 | 0x7F => {
                        buf.pop();
                        let _ = self.serial.write(b"\x08 \x08");
                    }
                    b if b.is_ascii_graphic() || b == b' ' => {
                        if buf.push(b as char).is_ok() {
                            let _ = self.serial.write(&[b]);
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }

    pub fn print(&mut self, s: &str) {
        let _ = self.serial.write(s.as_bytes());
    }
}
