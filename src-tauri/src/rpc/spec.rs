// This file was generated by xap-specs, do not edit manually

pub mod error {
    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, specta::Type)]
    pub struct Error(pub String);

    impl From<anyhow::Error> for Error {
        fn from(err: anyhow::Error) -> Self {
            Self(err.to_string())
        }
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod xap {
    use std::sync::{Arc, Mutex};

    use std::result::Result;
    use tauri::State;
    use uuid::Uuid;

    use crate::rpc::spec::error::Error;
    use crate::xap::client::XapClient;
    use crate::xap::spec::types::*;
    use crate::xap::spec::xap::*;

    #[tauri::command]
    #[specta::specta]
    pub fn xap_version(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<XapVersionResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, XapVersionRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn xap_capabilities(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<XapCapabilitiesFlags, Error> {
        state
            .lock()
            .unwrap()
            .query(id, XapCapabilitiesRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn xap_enabled_subsystem_capabilities(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<XapEnabledSubsystemCapabilitiesFlags, Error> {
        state
            .lock()
            .unwrap()
            .query(id, XapEnabledSubsystemCapabilitiesRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn xap_secure_status(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<XapSecureStatusResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, XapSecureStatusRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn xap_secure_unlock(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<(), Error> {
        state
            .lock()
            .unwrap()
            .query(id, XapSecureUnlockRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn xap_secure_lock(id: Uuid, state: State<'_, Arc<Mutex<XapClient>>>) -> Result<(), Error> {
        state
            .lock()
            .unwrap()
            .query(id, XapSecureLockRequest(()))
            .map_err(Into::into)
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod qmk {
    use std::sync::{Arc, Mutex};

    use std::result::Result;
    use tauri::State;
    use uuid::Uuid;

    use crate::rpc::spec::error::Error;
    use crate::xap::client::XapClient;
    use crate::xap::spec::qmk::*;
    use crate::xap::spec::types::*;

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_version(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkVersionResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkVersionRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_capabilities(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkCapabilitiesFlags, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkCapabilitiesRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_board_identifiers(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkBoardIdentifiersResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkBoardIdentifiersRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_board_manufacturer(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkBoardManufacturerResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkBoardManufacturerRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_product_name(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkProductNameResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkProductNameRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_config_blob_length(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkConfigBlobLengthResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkConfigBlobLengthRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_config_blob_chunk(
        id: Uuid,
        arg: u16,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkConfigBlobChunkResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkConfigBlobChunkRequest(arg))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_jump_to_bootloader(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkJumpToBootloaderResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkJumpToBootloaderRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_hardware_identifier(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkHardwareIdentifierResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkHardwareIdentifierRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn qmk_reinitialize_eeprom(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<QmkReinitializeEepromResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, QmkReinitializeEepromRequest(()))
            .map_err(Into::into)
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod keyboard {
    use std::sync::{Arc, Mutex};

    use std::result::Result;
    use tauri::State;
    use uuid::Uuid;

    use crate::rpc::spec::error::Error;
    use crate::xap::client::XapClient;
    use crate::xap::spec::keyboard::*;
    use crate::xap::spec::types::*;
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod user {
    use std::sync::{Arc, Mutex};

    use std::result::Result;
    use tauri::State;
    use uuid::Uuid;

    use crate::rpc::spec::error::Error;
    use crate::xap::client::XapClient;
    use crate::xap::spec::types::*;
    use crate::xap::spec::user::*;
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod keymap {
    use std::sync::{Arc, Mutex};

    use std::result::Result;
    use tauri::State;
    use uuid::Uuid;

    use crate::rpc::spec::error::Error;
    use crate::xap::client::XapClient;
    use crate::xap::spec::keymap::*;
    use crate::xap::spec::types::*;

    #[tauri::command]
    #[specta::specta]
    pub fn keymap_capabilities(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<KeymapCapabilitiesFlags, Error> {
        state
            .lock()
            .unwrap()
            .query(id, KeymapCapabilitiesRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn keymap_get_layer_count(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<KeymapGetLayerCountResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, KeymapGetLayerCountRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn keymap_get_keycode(
        id: Uuid,
        arg: KeymapGetKeycodeArg,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<KeymapGetKeycodeResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, KeymapGetKeycodeRequest(arg))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn keymap_get_encoder_keycode(
        id: Uuid,
        arg: KeymapGetEncoderKeycodeArg,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<KeymapGetEncoderKeycodeResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, KeymapGetEncoderKeycodeRequest(arg))
            .map_err(Into::into)
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod remapping {
    use std::sync::{Arc, Mutex};

    use std::result::Result;
    use tauri::State;
    use uuid::Uuid;

    use crate::rpc::spec::error::Error;
    use crate::xap::client::XapClient;
    use crate::xap::spec::remapping::*;
    use crate::xap::spec::types::*;

    #[tauri::command]
    #[specta::specta]
    pub fn remapping_capabilities(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<RemappingCapabilitiesFlags, Error> {
        state
            .lock()
            .unwrap()
            .query(id, RemappingCapabilitiesRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn remapping_get_layer_count(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<RemappingGetLayerCountResponse, Error> {
        state
            .lock()
            .unwrap()
            .query(id, RemappingGetLayerCountRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn remapping_set_keycode(
        id: Uuid,
        arg: RemappingSetKeycodeArg,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<(), Error> {
        state
            .lock()
            .unwrap()
            .query(id, RemappingSetKeycodeRequest(arg))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn remapping_set_encoder_keycode(
        id: Uuid,
        arg: RemappingSetEncoderKeycodeArg,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<(), Error> {
        state
            .lock()
            .unwrap()
            .query(id, RemappingSetEncoderKeycodeRequest(arg))
            .map_err(Into::into)
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod lighting {
    use std::sync::{Arc, Mutex};

    use std::result::Result;
    use tauri::State;
    use uuid::Uuid;

    use crate::rpc::spec::error::Error;
    use crate::xap::client::XapClient;
    use crate::xap::spec::lighting::*;
    use crate::xap::spec::types::*;

    #[tauri::command]
    #[specta::specta]
    pub fn lighting_capabilities(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<LightingCapabilitiesFlags, Error> {
        state
            .lock()
            .unwrap()
            .query(id, LightingCapabilitiesRequest(()))
            .map_err(Into::into)
    }

    #[allow(dead_code)]
    #[allow(unused_imports)]
    pub mod backlight {
        use std::sync::{Arc, Mutex};

        use std::result::Result;
        use tauri::State;
        use uuid::Uuid;

        use crate::rpc::spec::error::Error;
        use crate::xap::client::XapClient;
        use crate::xap::spec::lighting::backlight::*;
        use crate::xap::spec::types::*;

        #[tauri::command]
        #[specta::specta]
        pub fn backlight_capabilities(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<BacklightCapabilitiesFlags, Error> {
            state
                .lock()
                .unwrap()
                .query(id, BacklightCapabilitiesRequest(()))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn backlight_get_enabled_effects(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<BacklightGetEnabledEffectsResponse, Error> {
            state
                .lock()
                .unwrap()
                .query(id, BacklightGetEnabledEffectsRequest(()))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn backlight_get_config(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<BacklightConfig, Error> {
            state
                .lock()
                .unwrap()
                .query(id, BacklightGetConfigRequest(()))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn backlight_set_config(
            id: Uuid,
            arg: BacklightConfig,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<(), Error> {
            state
                .lock()
                .unwrap()
                .query(id, BacklightSetConfigRequest(arg))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn backlight_save_config(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<(), Error> {
            state
                .lock()
                .unwrap()
                .query(id, BacklightSaveConfigRequest(()))
                .map_err(Into::into)
        }
    }

    #[allow(dead_code)]
    #[allow(unused_imports)]
    pub mod rgblight {
        use std::sync::{Arc, Mutex};

        use std::result::Result;
        use tauri::State;
        use uuid::Uuid;

        use crate::rpc::spec::error::Error;
        use crate::xap::client::XapClient;
        use crate::xap::spec::lighting::rgblight::*;
        use crate::xap::spec::types::*;

        #[tauri::command]
        #[specta::specta]
        pub fn rgblight_capabilities(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<RgblightCapabilitiesFlags, Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgblightCapabilitiesRequest(()))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn rgblight_get_enabled_effects(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<RgblightGetEnabledEffectsResponse, Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgblightGetEnabledEffectsRequest(()))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn rgblight_get_config(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<RgbLightConfig, Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgblightGetConfigRequest(()))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn rgblight_set_config(
            id: Uuid,
            arg: RgbLightConfig,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<(), Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgblightSetConfigRequest(arg))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn rgblight_save_config(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<(), Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgblightSaveConfigRequest(()))
                .map_err(Into::into)
        }
    }

    #[allow(dead_code)]
    #[allow(unused_imports)]
    pub mod rgbmatrix {
        use std::sync::{Arc, Mutex};

        use std::result::Result;
        use tauri::State;
        use uuid::Uuid;

        use crate::rpc::spec::error::Error;
        use crate::xap::client::XapClient;
        use crate::xap::spec::lighting::rgbmatrix::*;
        use crate::xap::spec::types::*;

        #[tauri::command]
        #[specta::specta]
        pub fn rgbmatrix_capabilities(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<RgbmatrixCapabilitiesFlags, Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgbmatrixCapabilitiesRequest(()))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn rgbmatrix_get_enabled_effects(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<RgbmatrixGetEnabledEffectsResponse, Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgbmatrixGetEnabledEffectsRequest(()))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn rgbmatrix_get_config(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<RgbMatrixConfig, Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgbmatrixGetConfigRequest(()))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn rgbmatrix_set_config(
            id: Uuid,
            arg: RgbMatrixConfig,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<(), Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgbmatrixSetConfigRequest(arg))
                .map_err(Into::into)
        }

        #[tauri::command]
        #[specta::specta]
        pub fn rgbmatrix_save_config(
            id: Uuid,
            state: State<'_, Arc<Mutex<XapClient>>>,
        ) -> Result<(), Error> {
            state
                .lock()
                .unwrap()
                .query(id, RgbmatrixSaveConfigRequest(()))
                .map_err(Into::into)
        }
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod audio {
    use std::sync::{Arc, Mutex};

    use std::result::Result;
    use tauri::State;
    use uuid::Uuid;

    use crate::rpc::spec::error::Error;
    use crate::xap::client::XapClient;
    use crate::xap::spec::audio::*;
    use crate::xap::spec::types::*;

    #[tauri::command]
    #[specta::specta]
    pub fn audio_capabilities(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<AudioCapabilitiesFlags, Error> {
        state
            .lock()
            .unwrap()
            .query(id, AudioCapabilitiesRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn audio_get_config(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<AudioConfig, Error> {
        state
            .lock()
            .unwrap()
            .query(id, AudioGetConfigRequest(()))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn audio_set_config(
        id: Uuid,
        arg: AudioConfig,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<(), Error> {
        state
            .lock()
            .unwrap()
            .query(id, AudioSetConfigRequest(arg))
            .map_err(Into::into)
    }

    #[tauri::command]
    #[specta::specta]
    pub fn audio_save_config(
        id: Uuid,
        state: State<'_, Arc<Mutex<XapClient>>>,
    ) -> Result<(), Error> {
        state
            .lock()
            .unwrap()
            .query(id, AudioSaveConfigRequest(()))
            .map_err(Into::into)
    }
}

#[macro_export]
macro_rules! generate_specta_builder {
                (commands: [$($command:ident),*], events: [$($event:ident),*]) => {{
                    let specta_builder = tauri_specta::ts::builder()
                        .commands(tauri_specta::collect_commands![
                            crate::rpc::spec::xap::xap_version,
                            crate::rpc::spec::xap::xap_capabilities,
                            crate::rpc::spec::xap::xap_enabled_subsystem_capabilities,
                            crate::rpc::spec::xap::xap_secure_status,
                            crate::rpc::spec::xap::xap_secure_unlock,
                            crate::rpc::spec::xap::xap_secure_lock,
                            crate::rpc::spec::qmk::qmk_version,
                            crate::rpc::spec::qmk::qmk_capabilities,
                            crate::rpc::spec::qmk::qmk_board_identifiers,
                            crate::rpc::spec::qmk::qmk_board_manufacturer,
                            crate::rpc::spec::qmk::qmk_product_name,
                            crate::rpc::spec::qmk::qmk_config_blob_length,
                            crate::rpc::spec::qmk::qmk_config_blob_chunk,
                            crate::rpc::spec::qmk::qmk_jump_to_bootloader,
                            crate::rpc::spec::qmk::qmk_hardware_identifier,
                            crate::rpc::spec::qmk::qmk_reinitialize_eeprom,
                            crate::rpc::spec::keymap::keymap_capabilities,
                            crate::rpc::spec::keymap::keymap_get_layer_count,
                            crate::rpc::spec::keymap::keymap_get_keycode,
                            crate::rpc::spec::keymap::keymap_get_encoder_keycode,
                            crate::rpc::spec::remapping::remapping_capabilities,
                            crate::rpc::spec::remapping::remapping_get_layer_count,
                            crate::rpc::spec::remapping::remapping_set_keycode,
                            crate::rpc::spec::remapping::remapping_set_encoder_keycode,
                            crate::rpc::spec::lighting::lighting_capabilities,
                            crate::rpc::spec::lighting::backlight::backlight_capabilities,
                            crate::rpc::spec::lighting::backlight::backlight_get_enabled_effects,
                            crate::rpc::spec::lighting::backlight::backlight_get_config,
                            crate::rpc::spec::lighting::backlight::backlight_set_config,
                            crate::rpc::spec::lighting::backlight::backlight_save_config,
                            crate::rpc::spec::lighting::rgblight::rgblight_capabilities,
                            crate::rpc::spec::lighting::rgblight::rgblight_get_enabled_effects,
                            crate::rpc::spec::lighting::rgblight::rgblight_get_config,
                            crate::rpc::spec::lighting::rgblight::rgblight_set_config,
                            crate::rpc::spec::lighting::rgblight::rgblight_save_config,
                            crate::rpc::spec::lighting::rgbmatrix::rgbmatrix_capabilities,
                            crate::rpc::spec::lighting::rgbmatrix::rgbmatrix_get_enabled_effects,
                            crate::rpc::spec::lighting::rgbmatrix::rgbmatrix_get_config,
                            crate::rpc::spec::lighting::rgbmatrix::rgbmatrix_set_config,
                            crate::rpc::spec::lighting::rgbmatrix::rgbmatrix_save_config,
                            crate::rpc::spec::audio::audio_capabilities,
                            crate::rpc::spec::audio::audio_get_config,
                            crate::rpc::spec::audio::audio_set_config,
                            crate::rpc::spec::audio::audio_save_config,
                            $($command),*
                        ]).events(tauri_specta::collect_events![$($event),*]);

                    specta_builder
                }};
            }
