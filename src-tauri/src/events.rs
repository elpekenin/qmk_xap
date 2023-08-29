use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;
use xap_specs::protocol::{xap::XAPSecureStatus, BroadcastRaw};

use crate::aggregation::XAPDevice as XAPDeviceDTO;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Serialize, TS)]
#[serde(tag = "kind", content = "data")]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub(crate) enum FrontendEvent {
    NewDevice {
        device: XAPDeviceDTO,
    },
    RemovedDevice {
        id: Uuid,
    },
    SecureStatusChanged {
        id: Uuid,
        secure_status: XAPSecureStatus,
    },
    LogReceived {
        id: Uuid,
        log: String,
    },
    KeyTester {
        pressed: bool,
        row: u8,
        col: u8
    },
}

pub(crate) enum XAPEvent {
    ReceivedUserBroadcast {
        broadcast: BroadcastRaw,
        id: Uuid,
    },
    LogReceived {
        id: Uuid,
        log: String,
    },
    SecureStatusChanged {
        id: Uuid,
        secure_status: XAPSecureStatus,
    },

    NewDevice(Uuid),
    RemovedDevice(Uuid),
    AnnounceAllDevices,
    RxError,
    Exit,
}
