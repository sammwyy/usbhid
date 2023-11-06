extern crate usbhid;

use usbhid::{keyboard::Keyboard, prelude::Device};

fn main() {
    let device = Device::new("/dev/hidg0");
    let keyword = Keyboard::new(device, "us");
    keyword.type_string("Hello World!")
}
