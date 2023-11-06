extern crate usbhid;

use std::{thread::sleep, time::Duration};

use usbhid::prelude::{Device, Mouse};

fn main() {
    let device = Device::new("/dev/hidg1");
    let mouse = Mouse::new(device);

    // Move the mouse to the top left corner
    mouse.move_zero();
    sleep(Duration::from_millis(1000));

    // Move the mouse to x100 y100
    mouse.move_to(100, 100);
    sleep(Duration::from_millis(1000));

    // Move the mouse relative
    mouse.move_relative(127, 127);
    sleep(Duration::from_millis(1000));

    // Perform a right click
    mouse.right_click();
    sleep(Duration::from_millis(200));
}
