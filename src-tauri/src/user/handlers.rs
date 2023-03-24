use crate::{
    user::{
        gui::{self, Button, Screen, Slider},
        http::{home_assistant as ha, telegram as tg},
    },
    xap::hid::{XAPClient, XAPDevice},
};
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

use super::UserData;

pub(crate) fn slider(
    device: &XAPDevice,
    screen: &Screen,
    slider: &Slider,
    coord: u16,
    _user_data: &UserData,
) {
    let slider_id = screen.sliders.iter().position(|s| s == slider).unwrap();

    match screen.id {
        1 => match slider_id {
            0 => {
                let intensity = coord * 2 / 241;
                slider.draw(device, screen, intensity);
                ha::set_light_intensity(intensity);
            }

            _ => unreachable!(),
        },

        2 => match slider_id {
            0 => {
                let intensity = 5 - (coord * 6 / 321);
                slider.draw(device, screen, intensity);
                ha::set_light_intensity(intensity);
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}

pub(crate) fn button(device: &XAPDevice, screen: &Screen, button: &Button, user_data: &UserData) {
    let button_id = screen.buttons.iter().position(|b| b == button).unwrap();

    // Mark as pressed
    button.draw(device, screen, true);

    // Run its logic
    match screen.id {
        1 => match button_id {
            0 => {
                let json = ha::get_state("weather.forecast_casa");
                let attributes = json["attributes"].clone();

                screen.clear_text(device);
                let text = format!("Temperature: {} C", attributes["temperature"]).replace('"', "");
                screen.draw_text(device, text);
            }

            _ => unreachable!(),
        },

        2 => match button_id {
            0 => {
                screen.clear_text(device);
                screen.draw_text(device, "Test");
            }

            2 => {
                screen.clear_text(device);
                screen.draw_text(device, "Message sent");

                tg::text("QMK -> XAP -> TauriClient -> Telegram");
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }

    gui::clear(device, user_data);
}
