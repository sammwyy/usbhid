use std::{mem::transmute, thread::sleep, time::Duration};

use crate::{prelude::Device, utils::mathu16_min};

pub struct Mouse {
    device: Device,
}

impl Mouse {
    pub fn new(device: Device) -> Mouse {
        Mouse { device }
    }

    pub fn send_mouse_event(
        &self,
        buttons: u8,
        relative_x: u8,
        relative_y: u8,
        vertical_wheel_delta: u8,
        horizontal_wheel_delta: u8,
    ) {
        let mut data = [0; 5];
        data[0] = buttons;
        data[1] = relative_x;
        data[2] = relative_y;
        data[3] = vertical_wheel_delta & 0xff;
        data[4] = horizontal_wheel_delta & 0xff;
        self.device.write(&data).unwrap();
    }

    pub fn left_click(&self) {
        self.send_mouse_event(0x1, 0, 0, 0, 0);
        self.send_mouse_event(0x0, 0, 0, 0, 0);
    }

    pub fn right_click(&self) {
        self.send_mouse_event(0x1, 0, 0, 0, 0);
        self.send_mouse_event(0x2, 0, 0, 0, 0);
    }

    pub fn move_relative(&self, x: i8, y: i8) {
        let fixed_x = unsafe { transmute(x) };
        let fixed_y = unsafe { transmute(y) };

        self.send_mouse_event(0x0, fixed_x, fixed_y, 0, 0);
    }

    pub fn move_zero(&self) {
        let mut times = 0;

        loop {
            self.move_relative(-127, -127);
            sleep(Duration::from_millis(16));
            times = times + 1;

            if times >= 20 {
                break;
            }
        }
    }

    pub fn move_to(&self, x: u16, y: u16) {
        self.move_zero();

        let max_step = 127;
        let mut remaing_x = x;
        let mut remaing_z = y;

        loop {
            let step_x = mathu16_min(max_step, remaing_x);
            let step_y = mathu16_min(max_step, remaing_z);

            remaing_x = remaing_x - step_x;
            remaing_z = remaing_z - step_y;

            self.move_relative(step_x as i8, step_y as i8);
            sleep(Duration::from_millis(16));

            if remaing_x <= 0 && remaing_z <= 0 {
                break;
            }
        }
    }
}
