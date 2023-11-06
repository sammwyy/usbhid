# 🍓 USBHID

Rust Library for the USB HID protocol. Based on [zero-hid](https://github.com/thewh1teagle/zero-hid/tree/main).

## 🐛 Compatibility

| Compatible | Board                 | Port(s) |
| :--------: | --------------------- | ------- |
| ⚠️        | ESP32                 |          |
| ⚠️        | Raspberry Pi Pico     |          |
| ✅        | Raspberry Pi Zero 2w  | 🎩      |

> **Legend:** ✅ Compatible, ❌ Incompatible, ⚠️ Untested.
> **Ports:** 🎩 USB Hat, 📡 USB Port.

## ⚡ Features

- [X] Keyboard.
  - [X] Keydown.
  - [X] Keyup.
  - [X] Type string.
  - [X] Mod keys.
- [X] Mouse.
  - [X] Move to.
  - [X] Move relative.
  - [X] Left click.
  - [X] Right click.
  - [ ] Middle click.
  - [ ] Scroll Wheel.
- [ ] Mass storage.
  - [ ] Write file.
  - [ ] Read file.
  - [ ] Delete file.
- [ ] Ethernet.
  - [ ] Send packet.
  - [ ] Receive packet.
- [ ] Gamepad.
  - [ ] Button press.
  - [ ] Button release.
  - [ ] Joystick move.
- [ ] Printer.
  - [ ] Print text.
  - [ ] Print image.

## ⚙ Setup

Before using this library, you need to install a virtual USB HID device on your board.

```bash
# Clone the repository
git clone https://github.com/sammwyy/usbhid.git

# Go to the installer directory
cd usbhid/hid

# Install the virtual device
chmod +x install.sh && sudo ./install.sh
```

## 📚 Usage

**⌨ Keyboard:**

```rust
use usbhid::{keyboard:Keyboard, device::Device};

fn main() {
    // Install your virtual device before (See documentation)
    let mut device = Device::new("/dev/hidg0");
    let mut keyboard = Keyboard::new(device, "us");

    keyboard.type_string("Hello World!");
}
```

**🖱 Mouse:**

```rust
use usbhid::{mouse:Mouse, device::Device};

fn main() {
    // Install your virtual device before (See documentation)
    let mut device = Device::new("/dev/hidg1");
    let mut mouse = Mouse::new(device);

    mouse.move_to(100, 100);
    mouse.left_click();
}
```

## 🤝 Contributing

Contributions, issues and feature requests are welcome! Feel free to check [issues page](https://github.com/sammwyy/usbhid/issues).

## ❤️ Show your support

Give a ⭐️ if this project helped you! Or buy me a coffeelatte 🙌 on [Ko-fi](https://ko-fi.com/sammwy)

## 📝 License

Copyright © 2023 [Sammwy](https://github.com/sammwyy). This project is [MIT](LICENSE) licensed.
