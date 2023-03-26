mod gui;
mod http;
mod machine;
mod os;
mod spotify;

use crate::{
    user::{
        gui::{Button, Screen, Slider, SliderDirection},
        http::{home_assistant as ha, telegram as tg},
    },
    xap::hid::{XAPClient, XAPDevice},
};
use dotenvy::dotenv;
use log::{debug, info, trace, warn};
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};
use sysinfo::{System, SystemExt};
use uuid::Uuid;
use xap_specs::protocol::{BroadcastRaw, UserBroadcast};

// Custom data
#[derive(Default)]
pub struct UserData {
    pub last_song: String,
    pub last_url: String,
    pub counter: u32,
    pub sys: System,
    pub ram: u8,
    pub cpu: u8,
    pub screens: Vec<Screen>,
}

impl UserData {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
            screens: vec![
                // ILI9163
                Screen {
                    id: 0,
                    buttons: vec![],
                    sliders: vec![],
                },
                // ILI9341
                Screen {
                    id: 1,
                    buttons: vec![Button {
                        x: 150,
                        y: 100,
                        img: 0,
                        handler: Box::new(
                            |device: &XAPDevice,
                             screen: &Screen,
                             button: &Button,
                             msg: &UserBroadcast,
                             user_data: &UserData| {
                                tg::text("QMK -XAP-> TauriClient -HTTP-> Telegram");
                            },
                        ),
                    }],
                    sliders: vec![Slider {
                        direction: SliderDirection::Horizontal,
                        start: 190,
                        size: 50,
                        x: 150,
                        y: 100,
                        img_map: HashMap::from([
                            ("0", 0),
                            ("1", 1),
                            ("2", 2),
                            ("3", 3),
                            ("4", 4),
                            ("5", 5),
                        ]),
                        handler: Box::new(
                            |device: &XAPDevice,
                             screen: &Screen,
                             slider: &Slider,
                             msg: &UserBroadcast,
                             user_data: &UserData| {
                                let intensity = slider.coord(msg) * 6 / 321;
                                slider.draw(device, screen, intensity);
                                // ha::set_light_intensity(intensity);
                            },
                        ),
                    }],
                },
            ],
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

pub(crate) fn new_device(device: &XAPDevice, user_data: &UserData) {
    // Sleep is needed, so that screen is init'ed
    std::thread::sleep(std::time::Duration::from_millis(3000));
    gui::on_connect(device, user_data);
}

pub(crate) fn removed_device(_id: &Uuid, _user_data: &Arc<Mutex<UserData>>) {}

pub(crate) fn on_close(client: &XAPClient, user_data: &UserData) {
    gui::close(client, user_data);
}

pub(crate) fn broadcast_callback(
    broadcast: BroadcastRaw,
    device: &XAPDevice,
    user_data: &UserData,
) {
    // Parse raw data
    let msg: UserBroadcast = broadcast.into_xap_broadcast().unwrap();

    // info!("Received {msg:?}");

    // Clear any leftover graphics
    gui::clear(device, user_data);

    // Run logic, code assumes that sliders and buttons don't overlap, if they do button will have preference
    gui::handle(device, &msg, user_data);
}

pub(crate) fn housekeeping(client: &XAPClient, user_data: &mut UserData) {
    let devices = client.get_devices();

    let device = match devices.first() {
        Some(dev) => dev,
        None => {
            trace!("housekeeping: no device connected, quitting");
            return;
        }
    };

    // NOTE: ticks are 0.5s

    os::active_window(device, user_data);
    machine::stats(device, user_data);

    if user_data.counter % (5 * 2) == 0 {
        spotify::album_cover(device, user_data);
    }

    if user_data.counter % (60 * 1 * 2) == 0 {
        http::weather::draw(device, user_data);
    }

    (user_data.counter, _) = user_data.counter.overflowing_add(1);
}
