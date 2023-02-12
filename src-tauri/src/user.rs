mod gui;
mod handlers;
mod http;
mod os;

use crate::xap::hid::{XAPClient, XAPDevice};
use dotenvy::dotenv;
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;
use xap_specs::protocol::{BroadcastRaw, UserBroadcast};

// Hooks
pub(crate) fn pre_init() {
    // Read `.env` file
    dotenv().ok();

    // Make sure Home Assistant is running
    os::start_home_assistant();
}

pub(crate) fn on_device_connection(device: &XAPDevice) {
    // Sleep is needed, so that screen is init'ed
    std::thread::sleep(std::time::Duration::from_millis(3000));

    gui::init(device);
}

pub(crate) fn on_close(state: Arc<Mutex<XAPClient>>) {
    gui::close(&state);
}

pub(crate) fn broadcast_callback(broadcast: BroadcastRaw, id: Uuid, state: &Arc<Mutex<XAPClient>>) {
    // Parse raw data
    let msg: UserBroadcast = broadcast.into_xap_broadcast().unwrap();

    // info!("Received {msg:?}");

    // Clear any leftover graphics
    gui::clear(&state, &id);

    // Run logic, code assumes that sliders and buttons don't overlap, if they do button will have preference
    gui::handle(state, &id, &msg);
}

pub(crate) fn housekeeping(state: &Arc<Mutex<XAPClient>>) {
    os::spotify_cover(state);
}
