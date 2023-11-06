use serde_json::{Map, Value};

use core::panic;
use std::{collections::HashMap, str::FromStr};

use super::{keycodes::KeyCodes, keymods::KeyMods};

const US: &str = include_str!("mappings/us.json");

#[derive(Clone)]
pub struct KeyCombination {
    pub modifiers: Vec<KeyMods>,
    pub keys: Vec<KeyCodes>,
}

type RawLayout = Map<String, Value>;
type Layout = HashMap<String, KeyCombination>;

pub struct KeyMap {
    pub id: String,
    pub layout: Layout,
}

impl KeyMap {
    pub fn new(id: &str, raw_layout: RawLayout) -> KeyMap {
        let mut layout = HashMap::new();

        for (key, value) in raw_layout {
            let raw_mods = value.get("modifiers").unwrap().as_array().unwrap();
            let raw_keys = value.get("keys").unwrap().as_array().unwrap();

            let mut modifiers = Vec::new();

            for modifier in raw_mods {
                let modifier = modifier.as_str().unwrap();
                let modifier = KeyMods::from_str(modifier).unwrap();
                modifiers.push(modifier);
            }

            let mut key_codes = Vec::new();

            for key in raw_keys {
                let key = key.as_str().unwrap();
                let keycode = KeyCodes::from_str(key);

                if keycode.is_err() {
                    panic!("Key not found for {}", key.to_string());
                }

                key_codes.push(keycode.unwrap());
            }

            let key_combination = KeyCombination {
                modifiers,
                keys: key_codes,
            };

            layout.insert(key, key_combination);
        }

        KeyMap {
            id: id.to_string(),
            layout,
        }
    }

    pub fn get_char(&self, key: &str) -> Option<&KeyCombination> {
        let value = self.layout.get(key);
        value
    }

    pub fn get_char_safe(&self, key: &str) -> Option<&KeyCombination> {
        let value = self.get_char(key);

        if value.is_none() {
            self.get_char(format!("Key{}", key.to_uppercase()).as_str())
        } else {
            value
        }
    }

    pub fn get_string(&self, string: &str) -> Vec<&KeyCombination> {
        let mut result = Vec::new();

        for c in string.chars() {
            let key = c.to_string();
            let value = self.get_char_safe(&key);

            match value {
                Some(v) => result.push(v),
                None => panic!("Key not found for {}", key),
            }
        }

        result
    }
}

fn get_raw_layout(id: &str) -> Option<&str> {
    match id {
        "us" => Some(US),
        _ => None,
    }
}

pub fn get_layout(id: &str) -> Option<KeyMap> {
    let raw = get_raw_layout(id);

    match raw {
        Some(v) => {
            let raw_layout: RawLayout = serde_json::from_str(v).unwrap();
            Some(KeyMap::new(id, raw_layout))
        }
        None => None,
    }
}
