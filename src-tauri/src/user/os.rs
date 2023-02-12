mod linux;
mod windows;

use crate::xap::hid::XAPClient;
use log::warn;
use parking_lot::Mutex;
use std::sync::Arc;

const OS: &str = std::env::consts::OS;

pub fn start_home_assistant() {
    match OS {
        "linux" => linux::ha(),
        _ => warn!("start_home_assistant: not implemented for {}", OS),
    }
}

pub(crate) fn spotify_cover(state: &Arc<Mutex<XAPClient>>) {
    match OS {
        "windows" => windows::spotify(state),
        _ => warn!("spotify_cover: not implemented for {}", OS),
    }
}
