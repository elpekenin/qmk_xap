/// Information about the GUI
mod button;
pub mod draw;
mod screen;
mod slider;

pub use button::Button;
pub use screen::Screen;
pub use slider::{Slider, SliderDirection};

use crate::{xap::hid::XAPDevice, XAPClient};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;
use xap_specs::protocol::{painter::HSVColor, UserBroadcast};

// Assets size
pub const IMAGE_SIZE: u16 = 48;

// Color definitions
const HSV_WHITE: HSVColor = HSVColor {
    hue: 0,
    sat: 0,
    val: 255,
};
const HSV_BLACK: HSVColor = HSVColor {
    hue: 0,
    sat: 0,
    val: 0,
};

// Color palette
pub const BG_COLOR: HSVColor = HSV_BLACK;
pub const FG_COLOR: HSVColor = HSV_WHITE;

static SCREENS: Lazy<Vec<Screen>> = Lazy::new(|| {
    vec![
        // ILI9163
        Screen {
            width: 129,
            height: 128,
            id: 0,
            buttons: vec![],
            sliders: vec![],
        },
        // // ILI9341
        // Screen {
        //     width: 240,
        //     height: 320,
        //     id: 1,
        //     buttons: vec![
        //         Button {
        //             x: 50,
        //             y: 50,
        //             img: 0,
        //         }
        //     ],
        //     sliders: vec![
        //         // Slider {
        //         //     direction: SliderDirection::Horizontal,
        //         //     start: 270,
        //         //     size: 50,
        //         //     x: 50,
        //         //     y: 120,
        //         //     img_map: HashMap::from([
        //         //         ("0", 0),
        //         //         ("1", 1),
        //         //     ]),
        //         // }
        //     ],
        // },

        // // ILI9486
        // Screen {
        //     width: 480,
        //     height: 320,
        //     id: 2,
        //     buttons: vec![
        //         Button {
        //             x: 150,
        //             y: 150,
        //             img: 6,
        //         }
        //     ],
        //     sliders: vec![
        //         Slider {
        //             direction: SliderDirection::Vertical,
        //             start: 400,
        //             size: 80,
        //             x: 240,
        //             y: 160,
        //             img_map: HashMap::from([
        //                 ("0", 0),
        //                 ("1", 1),
        //                 ("2", 2),
        //                 ("3", 3),
        //                 ("4", 4),
        //                 ("5", 5),
        //             ]),
        //         }
        //     ],
        // }
    ]
});

pub fn on_connect(device: &XAPDevice) {
    for screen in &*SCREENS {
        // Clear screen
        draw::rect(
            device,
            screen.id,
            0,
            0,
            screen.width,
            screen.height,
            BG_COLOR,
            true,
        );

        // Show connection
        draw::text_recolor(device, screen.id, 15, 15, 0, FG_COLOR, BG_COLOR, "Tauri ON");

        // Print buttons
        for button in &screen.buttons {
            draw::image_recolor(
                &device, screen.id, button.x, button.y, button.img, FG_COLOR, BG_COLOR,
            );
        }
    }
}

pub(crate) fn close(state: &Arc<Mutex<XAPClient>>) {
    for device in state.clone().lock().get_devices() {
        for screen in &*SCREENS {
            // Clear screen
            draw::rect(
                device,
                screen.id,
                0,
                0,
                screen.width,
                screen.height,
                BG_COLOR,
                true,
            );

            // Show text
            draw::text_recolor(
                device,
                screen.id,
                15,
                15,
                0,
                FG_COLOR,
                BG_COLOR,
                "Tauri OFF",
            );
        }
    }
}

pub(crate) fn clear(state: &Arc<Mutex<XAPClient>>, id: &Uuid) {
    for screen in &*SCREENS {
        for button in &screen.buttons {
            button.draw(state.lock().get_device(id).unwrap(), screen, false);
        }

        for slider in &screen.sliders {
            slider.clear(state.lock().get_device(id).unwrap(), screen);
        }
    }
}

pub(crate) fn handle(state: &Arc<Mutex<XAPClient>>, id: &Uuid, msg: &UserBroadcast) {
    for screen in &*SCREENS {
        screen.handle(state, &id, &msg);
    }
}
