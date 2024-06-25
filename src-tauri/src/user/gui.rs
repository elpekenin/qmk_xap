/// Information about the GUI

#[allow(unused)]
mod button;

#[allow(unused)]
pub mod draw;

mod screen;

#[allow(unused)]
mod slider;

pub use button::Button;
pub use screen::Screen;
pub use slider::{Slider, SliderDirection};

use crate::xap::hid::{XAPClient, XAPDevice};
use xap_specs::protocol::painter::HSVColor;

use super::UserData;

// Assets size
pub const IMAGE_SIZE: u16 = 48;
pub const FONT_SIZES: [u16; 2] = [18, 15]; // font height in pixels

// Color definitions
pub const HSV_WHITE: HSVColor = HSVColor {
    hue: 0,
    sat: 0,
    val: 255,
};
pub const HSV_BLACK: HSVColor = HSVColor {
    hue: 0,
    sat: 0,
    val: 0,
};
pub const HSV_RED: HSVColor = HSVColor {
    hue: 0,
    sat: 255,
    val: 255,
};

// Color palette
pub const BG_COLOR: HSVColor = HSV_BLACK;
pub const FG_COLOR: HSVColor = HSV_WHITE;

pub fn on_connect(device: &XAPDevice, user_data: &UserData) {
    // Draws buttons
    clear(device, user_data);
}

pub(crate) fn close(client: &XAPClient, user_data: &UserData) {
    for device in client.get_devices() {
        draw::stop_scrolling_text(device, user_data.playing_token);
        draw::stop_scrolling_text(device, user_data.artist_token);
        draw::stop_scrolling_text(device, user_data.active_window_token);

        for screen in &user_data.screens {
            draw::clear(device, screen.id);

            // Show goodbye text
            draw::text_recolor(device, screen.id, 15, 15, 0, FG_COLOR, BG_COLOR, ":(");
        }
    }
}

pub(crate) fn clear(device: &XAPDevice, user_data: &UserData) {
    for screen in &user_data.screens {
        screen.clear(device);
    }
}
