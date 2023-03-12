mod gui;
mod handlers;
mod http;
mod os;
mod spotify;

use crate::xap::hid::{XAPClient, XAPDevice};
use dotenvy::dotenv;
use log::{debug, info, trace, warn};
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;
use xap_specs::protocol::{BroadcastRaw, UserBroadcast};

// Custom data
#[derive(Default)]
pub struct UserData {
    pub last_song: String,
    pub connected: bool,
    // up to 256/2 => 128 seconds timer
    pub counter: u8,
}

// Hooks
pub(crate) fn pre_init() {
    // Read `.env` file
    dotenv().ok();

    // Make sure Home Assistant is running
    os::start_home_assistant();

    // Login on Spotify
    spotify::login();
}

pub(crate) fn new_device(device: &XAPDevice, user_data: &Arc<Mutex<UserData>>) {
    // Sleep is needed, so that screen is init'ed
    std::thread::sleep(std::time::Duration::from_millis(3000));
    gui::on_connect(device);
    user_data.lock().connected = true;
}

pub(crate) fn removed_device(_id: &Uuid, user_data: &Arc<Mutex<UserData>>) {
    user_data.lock().connected = false;
}

pub(crate) fn on_close(state: Arc<Mutex<XAPClient>>) {
    gui::close(&state);
}

pub(crate) fn broadcast_callback(broadcast: BroadcastRaw, id: Uuid, state: &Arc<Mutex<XAPClient>>) {
    // Parse raw data
    let msg: UserBroadcast = broadcast.into_xap_broadcast().unwrap();

    // info!("Received {msg:?}");

    // Clear any leftover graphics
    gui::clear(state, &id);

    // Run logic, code assumes that sliders and buttons don't overlap, if they do button will have preference
    gui::handle(state, &id, &msg);
}

pub(crate) fn housekeeping(client: &XAPClient, user_data: &Arc<Mutex<UserData>>) {
    let mut user_data = user_data.lock();

    if !user_data.connected {
        trace!("housekeeping: no device connected, quitting");
        user_data.last_song = String::new();
        return;
    }

    // modulo to prevent overflow
    user_data.counter = (user_data.counter + 1) % 255;

    // ticks are 0.5s
    if user_data.counter % 10 == 0 {
        spotify::album_cover(client.get_devices()[0], &mut user_data);
    }
}
