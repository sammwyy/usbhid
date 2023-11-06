use crate::prelude::Device;

use super::{
    keycodes::KeyCodes,
    keymaps::{get_layout, KeyCombination, KeyMap},
    keymods::KeyMods,
};

pub struct Keyboard {
    device: Device,
    keymap: KeyMap,
}

impl Keyboard {
    pub fn new(device: Device, layout: &str) -> Keyboard {
        let keymap = get_layout(layout).unwrap();
        Keyboard { device, keymap }
    }

    pub fn type_string(&self, string: &str) {
        let combinations = self.keymap.get_string(string);
        for combination in combinations {
            self.press_combination(combination.clone());
            self.release_keys();
        }
    }

    pub fn type_string_nl(&self, string: &str) {
        self.type_string(string);
        self.press_combination(KeyCombination {
            modifiers: Vec::new(),
            keys: vec![KeyCodes::KeyEnter],
        });
        self.release_keys();
    }

    pub fn press_combination(&self, combination: KeyCombination) {
        let mut data = [0; 8];

        for (i, modifier) in combination.modifiers.iter().enumerate() {
            data[i] = *modifier as u8;
        }

        for (i, key) in combination.keys.iter().enumerate() {
            data[i + 2] = *key as u8;
        }

        self.device.write(&data).unwrap();
    }

    pub fn press_keys_with_mods(&self, modifiers: Vec<KeyMods>, keys: Vec<KeyCodes>) {
        self.press_combination(KeyCombination { modifiers, keys })
    }

    pub fn press_with_mods(&self, modifiers: Vec<KeyMods>, key: KeyCodes) {
        let keys = vec![key];
        self.press_combination(KeyCombination { modifiers, keys })
    }

    pub fn press_keys(&self, keys: Vec<KeyCodes>) {
        self.press_combination(KeyCombination {
            modifiers: Vec::new(),
            keys,
        })
    }

    pub fn press(&self, key: KeyCodes) {
        let keys = vec![key];
        self.press_combination(KeyCombination {
            modifiers: Vec::new(),
            keys,
        })
    }

    pub fn release_keys(&self) {
        self.device.write(&[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    }
}
