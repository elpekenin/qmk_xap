mod linux;
mod windows;

use crate::user::gui::{self, HSV_BLACK, HSV_WHITE};
use active_win_pos_rs::get_active_window;
use log::warn;
use xap_specs::protocol::painter::HSVColor;

use crate::{user::UserData, xap::hid::XAPDevice};

const OS: &str = std::env::consts::OS;
const FONT: u8 = 0;
pub const FONT_SIZE: u16 = gui::FONT_SIZES[FONT as usize];
const SCREEN_ID: u8 = 1;
pub const Y: u16 = 15;
const FG_COLOR: HSVColor = HSV_WHITE;
const BG_COLOR: HSVColor = HSV_BLACK;

pub fn start_home_assistant() {
    match OS {
        "linux" => linux::ha(),
        _ => warn!("start_home_assistant: not implemented for {}", OS),
    }
}

fn __active_window() -> Option<String> {
    let window = get_active_window().ok()?;

    Some(window.process_name.replace(['-'], " "))
}

pub fn active_window(device: &XAPDevice, user_data: &mut UserData) {
    let output = match OS {
        "linux" | "windows" => __active_window(),
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

    // Same window, don't update
    if user_data.active_window == text {
        return;
    }

    // Clear previous string
    let screen_width = gui::draw::geometry(device, SCREEN_ID).width;
    gui::draw::stop_scrolling_text(device, user_data.active_window_token);
    gui::draw::rect(
        device,
        SCREEN_ID,
        0,
        Y,
        screen_width,
        Y + FONT_SIZE,
        BG_COLOR,
        true,
    );

    // Update variable and draw new text
    user_data.active_window = text.clone();
    user_data.active_window_token =
        gui::draw::centered_or_scrolling_text(device, SCREEN_ID, Y, FONT, text);
}
