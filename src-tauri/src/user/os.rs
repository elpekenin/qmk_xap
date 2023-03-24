mod linux;
mod windows;

use crate::user::gui::{self, FONT_SIZE, HSV_BLACK, HSV_WHITE};
use log::warn;

use crate::{user::UserData, xap::hid::XAPDevice};

const OS: &str = std::env::consts::OS;

pub fn start_home_assistant() {
    match OS {
        "linux" => linux::ha(),
        _ => warn!("start_home_assistant: not implemented for {}", OS),
    }
}

pub fn active_window(device: &XAPDevice, _user_data: &mut UserData) {
    let output = match OS {
        "windows" => windows::active_window(),
        _ => {
            warn!("active_window: not implemented for {}", OS);
            return;
        }
    };

    let text = match output {
        Some(text) => text,
        None => {
            return;
        }
    };

    let screen_id = 1;
    let font = 0;

    let width = gui::draw::geometry(device, screen_id).width;
    let x = width - gui::draw::text_width(device, font, text.clone());
    let y = 0;

    gui::draw::rect(device, screen_id, 0, 0, width, FONT_SIZE, HSV_BLACK, true);
    gui::draw::text_recolor(device, screen_id, x, y, font, HSV_WHITE, HSV_BLACK, text);
}
