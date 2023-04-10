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

pub fn active_window(device: &XAPDevice, user_data: &mut UserData) {
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

    // Same window, don't update
    if user_data.active_window == text {
        return;
    }

    let screen_id = 1;
    let y = 0;
    let font = 0;
    let fg_color = HSV_WHITE;
    let bg_color = HSV_BLACK;

    // Get screen's size
    let screen_width = gui::draw::geometry(device, screen_id).width;

    // Clear previous string
    let x = screen_width - gui::draw::text_width(device, font, user_data.active_window.clone());
    gui::draw::rect(device, screen_id, x, y, screen_width, y + FONT_SIZE, bg_color.clone(), true);

    // Update variable and draw new text
    user_data.active_window = text.clone();
    let x = screen_width - gui::draw::text_width(device, font, user_data.active_window.clone());
    gui::draw::text_recolor(device, screen_id, x, y, font, fg_color, bg_color, text);
}
