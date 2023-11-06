use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, EnumString)]
pub enum KeyMods {
    ModLeftCtrl = 0x01,
    ModLeftShift = 0x02,
    ModLeftAlt = 0x04,
    ModLeftGui = 0x08,
    ModRightCtrl = 0x10,
    ModRightShift = 0x20,
    ModRightAlt = 0x40,
    ModRightGui = 0x80,
}

impl Into<u8> for KeyMods {
    fn into(self) -> u8 {
        self as u8
    }
}
