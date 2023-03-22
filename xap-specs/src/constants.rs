pub mod keycode;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::Serialize;

use crate::constants::keycode::{read_xap_keycodes, XAPKeyCode};
use crate::error::XAPResult;

#[derive(Debug, Clone, Serialize)]
pub struct XAPConstants {
    pub keycodes: HashMap<u16, XAPKeyCode>,
}

impl XAPConstants {
    pub fn new(specs_path: PathBuf) -> XAPResult<Self> {
        Ok(Self {
            keycodes: read_xap_keycodes(specs_path)?,
        })
    }

    #[must_use]
    pub fn get_keycode(&self, code: u16) -> XAPKeyCode {
        let mut keycode = self
            .keycodes
            .get(&code)
            .cloned()
            .unwrap_or_else(|| XAPKeyCode::new_custom(code));

        // TODO: Dynamic ranges from hjson files
        keycode.label = match code {
            0 => Some("KC_NO".to_string()),
            1 => Some("".to_string()),
            0x5220..=0x523F => Some(format!("MO({})", code - 0x5220)), // MO (0 makes no sense...)
            0x5700..=0x57FF => Some(format!("TD({})", code - 0x5700)), // TD
            _ => keycode.label.clone(),                                // Keep value
        };

        keycode
    }
}
