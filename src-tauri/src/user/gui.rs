/// Information about the GUI
mod button;
pub mod draw;
mod screen;
mod slider;

pub use button::Button;
pub use screen::Screen;
pub use slider::{Slider, SliderDirection};

use crate::xap::hid::{XAPClient, XAPDevice};
use xap_specs::protocol::{painter::HSVColor, UserBroadcast};

use super::UserData;

// Assets size
pub const IMAGE_SIZE: u16 = 48;
pub const FONT_SIZE: u16 = 15;

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

// Color palette
pub const BG_COLOR: HSVColor = HSV_BLACK;
pub const FG_COLOR: HSVColor = HSV_WHITE;

pub fn on_connect(device: &XAPDevice, user_data: &UserData) {
    for screen in &user_data.screens {
        draw::clear(device, screen.id);
    }

    // Draws buttons
    clear(device, user_data);
}

pub(crate) fn close(client: &XAPClient, user_data: &UserData) {
    for device in client.get_devices() {
        for screen in &user_data.screens {
            draw::clear(device, screen.id);

            // Show text
            draw::text_recolor(device, screen.id, 15, 15, 0, FG_COLOR, BG_COLOR, ":(");
        }
    }
}

pub(crate) fn clear(device: &XAPDevice, user_data: &UserData) {
    for screen in &user_data.screens {
        for button in &screen.buttons {
            button.draw(device, screen, false);
        }

        for slider in &screen.sliders {
            slider.clear(device, screen);
        }
    }
}

pub(crate) fn handle(device: &XAPDevice, msg: &UserBroadcast, user_data: &UserData) {
    for screen in &user_data.screens {
        screen.handle(device, msg, user_data);
    }
}
