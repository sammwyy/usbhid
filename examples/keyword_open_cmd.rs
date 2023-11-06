extern crate usbhid;

use std::{thread::sleep, time::Duration};

use usbhid::{
    keyboard::Keyboard,
    prelude::{Device, KeyCodes, KeyMods},
};

fn main() {
    let device = Device::new("/dev/hidg0");
    let keyword = Keyboard::new(device, "us");

    keyword.press_keys_with_mods(vec![(KeyMods::ModLeftGui)], vec![(KeyCodes::KeyR)]);
    sleep(Duration::from_millis(200));
    keyword.type_string_nl("cmd");
    sleep(Duration::from_millis(200));
}
