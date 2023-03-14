use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;
use xap_specs::{
    constants::keycode::{XAPKeyCode, XAPKeyCodeConfig},
    protocol::{qmk::QMKBoardIdentifiers, xap::XAPSecureStatus},
};

#[derive(Clone, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct XAPDevice {
    pub id: Uuid,
    pub info: XAPDeviceInfo,
    pub keymap: Vec<Vec<Vec<XAPKeyCodeConfig>>>,
    pub secure_status: XAPSecureStatus,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct XAPDeviceInfo {
    pub xap: XAPInfo,
    pub qmk: QMKInfo,
    pub features: FeaturesInfo,
    pub keymap: Option<KeymapInfo>,
    pub remap: Option<RemapInfo>,
    pub lighting: Option<LightingInfo>,
    pub split: Option<SplitInfo>,
}

#[derive(Debug, Deserialize, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct FeaturesInfo {
    pub audio: Option<bool>,
    pub backlight: Option<bool>,
    pub bootmagic: Option<bool>,
    pub console: Option<bool>,
    pub encoder: Option<bool>,
    pub extrakey: Option<bool>,
    pub mousekey: Option<bool>,
    pub nkro: Option<bool>,
    pub quantum_painter: Option<bool>,
    pub rgb_matrix: Option<bool>,
    pub tap_dance: Option<bool>,
    pub unicode: Option<bool>,
    pub usbpd: Option<bool>,
    pub wpm: Option<bool>,
    pub xap: Option<bool>,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct XAPInfo {
    pub version: String,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct QMKInfo {
    pub version: String,
    pub board_ids: QMKBoardIdentifiers,
    pub manufacturer: String,
    pub product_name: String,
    pub config: String,
    pub hardware_id: String,
    pub jump_to_bootloader_enabled: bool,
    pub eeprom_reset_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct SplitInfo {
    pub enabled: bool,
    pub main: String,
}

#[derive(Deserialize, Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct Matrix {
    pub cols: u8,
    pub rows: u8,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct KeymapInfo {
    pub matrix: Matrix,
    pub layer_count: Option<u8>,
    pub get_keycode_enabled: bool,
    pub get_encoder_keycode_enabled: bool,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct RemapInfo {
    pub layer_count: Option<u8>,
    pub set_keycode_enabled: bool,
    pub set_encoder_keycode_enabled: bool,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct LightingInfo {
    pub backlight: Option<BacklightInfo>,
    pub rgblight: Option<RGBLightInfo>,
    pub rgbmatrix: Option<RGBMatrixInfo>,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct BacklightInfo {
    pub effects: Option<Vec<u8>>,
    pub get_config_enabled: bool,
    pub set_config_enabled: bool,
    pub save_config_enabled: bool,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct RGBLightInfo {
    pub effects: Option<Vec<u8>>,
    pub get_config_enabled: bool,
    pub set_config_enabled: bool,
    pub save_config_enabled: bool,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct RGBMatrixInfo {
    pub effects: Option<Vec<u8>>,
    pub get_config_enabled: bool,
    pub set_config_enabled: bool,
    pub save_config_enabled: bool,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct XAPKeyCodeCategory {
    name: String,
    codes: Vec<XAPKeyCode>,
}

#[derive(Debug, Serialize, TS, Clone)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct XAPConstants {
    keycodes: Vec<XAPKeyCodeCategory>,
}

impl From<xap_specs::constants::XAPConstants> for XAPConstants {
    fn from(constants: xap_specs::constants::XAPConstants) -> Self {
        let keycodes =
            constants
                .keycodes
                .into_iter()
                .fold(HashMap::new(), |mut category, (_, keycode)| {
                    category
                        .entry(keycode.group.clone().unwrap_or_else(|| "other".to_owned()))
                        .or_insert(Vec::new())
                        .push(keycode);

                    category
                });

        let keycodes = keycodes
            .into_iter()
            .map(|(name, codes)| XAPKeyCodeCategory { name, codes })
            .collect();

        Self { keycodes }
    }
}
