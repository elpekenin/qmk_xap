mod gui;
mod http;
mod machine;
mod os;
mod spotify;
mod time;

use crate::{
    user::{
        gui::{Button, Screen, Slider, SliderDirection, IMAGE_SIZE},
        http::{home_assistant as ha, telegram as tg},
    },
    xap::hid::{XAPClient, XAPDevice},
};
use chrono::{DateTime, Local};
use dotenvy::dotenv;
use std::collections::HashMap;
use sysinfo::{System, SystemExt, User};
use uuid::Uuid;
use xap_specs::protocol::{
    BroadcastRaw,
    UserBroadcast::{self, *},
};

// Custom data
#[derive(Default)]
pub struct UserData {
    // common data
    pub connected: bool,
    pub counter: u32,
    pub screens: Vec<Screen>,

    // github
    pub notifications: u8,

    // machine
    pub cpu: u8,
    pub ram: u8,
    pub sys: System,

    // os
    pub active_window: String,
    pub active_window_token: Option<u8>,

    // spotify
    pub artist_token: Option<u8>,
    pub last_song: String,
    pub last_url: String,
    pub no_song_token: Option<u8>,
    pub song_token: Option<u8>,

    // time
    pub time: DateTime<Local>,
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
                    buttons: vec![
                        // Button {
                        //     x: 320 - IMAGE_SIZE,
                        //     y: 240 - 3 * IMAGE_SIZE,
                        //     img: 0,
                        //     handler: Box::new(
                        //         |_device: &XAPDevice,
                        //         _screen: &Screen,
                        //         _button: &Button,
                        //         _msg: &UserBroadcast,
                        //         _user_data: &UserData| {
                        //             tg::text("QMK -XAP-> TauriClient -HTTP-> Telegram");
                        //         },
                        //     ),
                        // }
                    ],
                    sliders: vec![
                        // Slider {
                        //     direction: SliderDirection::Horizontal,
                        //     start: 190,
                        //     size: 50,
                        //     x: 320 - 3 * IMAGE_SIZE,
                        //     y: 240 - 2 * IMAGE_SIZE,
                        //     img_map: HashMap::from([
                        //         ("0", 1),
                        //         ("1", 3),
                        //         ("2", 4),
                        //         ("3", 5),
                        //         ("4", 6),
                        //         ("5", 2),
                        //     ]),
                        //     handler: Box::new(
                        //         |device: &XAPDevice,
                        //         screen: &Screen,
                        //         slider: &Slider,
                        //         msg: &UserBroadcast,
                        //         _user_data: &UserData| {
                        //             let intensity = slider.coord(msg) * 6 / 321;
                        //             slider.draw(device, screen, intensity);
                        //             // ha::set_light_intensity(intensity);
                        //         },
                        //     ),
                        // }
                    ],
                },
            ],
            notifications: 1,  // forces cleanup on boot
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
    gui::on_connect(device, user_data);
}

pub(crate) fn removed_device(_id: &Uuid, _user_data: &mut UserData) {}

pub(crate) fn on_close(client: &XAPClient, user_data: &mut UserData) {
    gui::close(client, user_data);
}

fn get_display(user_data: &UserData, screen_id: u8) -> &Screen {
    user_data
        .screens
        .iter()
        .find(|s| s.id == screen_id)
        .expect("Unknown screen_id, WTF did you do?")
}

pub(crate) fn broadcast_callback(
    broadcast: BroadcastRaw,
    device: &XAPDevice,
    user_data: &mut UserData,
) {
    // Parse raw data
    let msg: UserBroadcast = if let Ok(m) = broadcast.into_xap_broadcast() {
        m
    } else {
        log::error!("Couldn't parse broadcast into user broadcast");
        return;
    };

    match msg {
        ScreenPressed(msg) => get_display(user_data, msg.screen_id).handle(device, &msg, user_data),
        ScreenReleased(msg) => get_display(user_data, msg.screen_id).clear(device),
        // nothing done for other messages
        LayerChanged(msg) => {}
        KeyEvent(msg) => {}
        Shutdown(msg) => {
            if msg.bootloader != 0 {
                std::process::exit(0)
            }
        }
    };
}

pub(crate) fn housekeeping(client: &XAPClient, user_data: &mut UserData) {
    let devices = client.get_devices();

    let device = match devices.first() {
        Some(dev) => dev,
        None => {
            log::trace!("housekeeping: no device connected, quitting");
            return;
        }
    };

    if !user_data.connected {
        log::info!("Waiting until displays are clear");
        return;
    }

    // NOTE: ticks are 0.5s

    machine::stats(device, user_data);
    os::active_window(device, user_data);
    time::draw(device, user_data);

    if user_data.counter % (5 * 2) == 0 {
        spotify::album_cover(device, user_data);
    }

    if user_data.counter % (30 * 2) == 0 {
        http::github::draw(device, user_data);
    }

    if user_data.counter % (60 * 10 * 2) == 0 {
        http::weather::draw(device, user_data);
    }

    (user_data.counter, _) = user_data.counter.overflowing_add(1);
}

pub fn get_var(name: impl Into<String>) -> String {
    let name = name.into();

    std::env::var(name.clone()).unwrap_or_else(|_| panic!("{name} not found"))
}
