use core::cell::RefCell;
use cortex_m::peripheral::NVIC;
use critical_section::Mutex;
use rp235x_hal as hal;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::SerialPort;

use hal::pac::interrupt;

static USB_DEVICE: Mutex<RefCell<Option<UsbDevice<'static, hal::usb::UsbBus>>>> =
    Mutex::new(RefCell::new(None));
static USB_SERIAL: Mutex<RefCell<Option<SerialPort<'static, hal::usb::UsbBus>>>> =
    Mutex::new(RefCell::new(None));
static RX_QUEUE: Mutex<RefCell<heapless::Deque<u8, 256>>> =
    Mutex::new(RefCell::new(heapless::Deque::new()));

pub fn init_usb(bus: &'static UsbBusAllocator<hal::usb::UsbBus>) {
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

    critical_section::with(|cs| {
        USB_DEVICE.borrow(cs).replace(Some(device));
        USB_SERIAL.borrow(cs).replace(Some(serial));
    });

    unsafe {
        NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    }
}

#[interrupt]
fn USBCTRL_IRQ() {
    critical_section::with(|cs| {
        let mut dev_ref = USB_DEVICE.borrow_ref_mut(cs);
        let mut ser_ref = USB_SERIAL.borrow_ref_mut(cs);

        if let (Some(device), Some(serial)) = (dev_ref.as_mut(), ser_ref.as_mut()) {
            if device.poll(&mut [serial]) {
                let mut buf = [0u8; 64];
                if let Ok(count) = serial.read(&mut buf) {
                    let mut queue = RX_QUEUE.borrow_ref_mut(cs);
                    for &b in &buf[..count] {
                        let _ = queue.push_back(b);
                    }
                }
            }
        }
    });
}

pub fn read_byte() -> Option<u8> {
    critical_section::with(|cs| RX_QUEUE.borrow_ref_mut(cs).pop_front())
}

pub fn poll_line(buf: &mut heapless::String<64>) -> Option<heapless::String<64>> {
    let mut result = None;

    critical_section::with(|cs| {
        let mut queue = RX_QUEUE.borrow_ref_mut(cs);
        while let Some(b) = queue.pop_front() {
            match b {
                b'\r' | b'\n' => {
                    if !buf.is_empty() {
                        result = Some(buf.clone());
                        buf.clear();
                        drop(queue);
                        write_bytes(cs, b"\r\n");
                        return;
                    }
                }
                0x08 | 0x7F => {
                    buf.pop();
                }
                b if b.is_ascii_graphic() || b == b' ' => {
                    let _ = buf.push(b as char);
                }
                _ => {}
            }
        }
    });

    result
}

fn write_bytes(cs: critical_section::CriticalSection, s: &[u8]) {
    let mut ser_ref = USB_SERIAL.borrow_ref_mut(cs);
    if let Some(serial) = ser_ref.as_mut() {
        let _ = serial.write(s);
    }
}

pub fn print(s: &str) {
    critical_section::with(|cs| write_bytes(cs, s.as_bytes()));
}
