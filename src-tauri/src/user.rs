mod gui;
mod handlers;
mod http;
mod machine;
mod os;
mod spotify;

use crate::xap::hid::{XAPClient, XAPDevice};
use dotenvy::dotenv;
use log::{debug, info, trace, warn};
use parking_lot::Mutex;
use std::sync::Arc;
use sysinfo::{System, SystemExt};
use uuid::Uuid;
use xap_specs::protocol::{BroadcastRaw, UserBroadcast};

// Custom data
#[derive(Default)]
pub struct UserData {
    pub last_song: String,
    pub last_url: String,
    pub connected: bool,
    pub counter: u32,
    pub sys: System,
}

impl UserData {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
            ..Default::default()
        }
    }
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

pub(crate) fn housekeeping(client: &XAPClient, user_data: &mut UserData) {
    if !user_data.connected {
        trace!("housekeeping: no device connected, quitting");
        *user_data = UserData::new();
        return;
    }

    let devices = client.get_devices();

    let device = match devices.first() {
        Some(dev) => dev,
        None => {
            user_data.connected = false;
            return;
        }
    };

    // NOTE: ticks are 0.5s

    os::active_window(device, user_data);
    machine::stats(device, user_data);

    if user_data.counter % (5 * 2) == 0 {
        spotify::album_cover(device, user_data);
    }

    // once every 10 mins
    if user_data.counter % (60 * 10 * 2) == 0 {
        http::weather::draw(device, user_data);
    }

    (user_data.counter, _) = user_data.counter.overflowing_add(1);
}
