use crate::{
    user::{
        gui::{self, Button, Screen, Slider},
        http::{home_assistant as ha, telegram as tg},
    },
    xap::hid::XAPClient,
};
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) fn slider(
    state: &Arc<Mutex<XAPClient>>,
    id: &Uuid,
    screen: &Screen,
    slider: &Slider,
    coord: u16,
) {
    let slider_id = screen.sliders.iter().position(|s| s == slider).unwrap();

    match screen.id {
        1 => match slider_id {
            0 => {
                let intensity = (coord * 2 / 241) as u16;
                slider.draw(state.lock().get_device(id).unwrap(), screen, intensity);
                ha::set_light_intensity(intensity);
            }

            _ => unreachable!(),
        },

        2 => match slider_id {
            0 => {
                let intensity = 5 - (coord * 6 / 321) as u16;
                slider.draw(state.lock().get_device(id).unwrap(), screen, intensity);
                ha::set_light_intensity(intensity);
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}

pub(crate) fn button(state: &Arc<Mutex<XAPClient>>, id: &Uuid, screen: &Screen, button: &Button) {
    let button_id = screen.buttons.iter().position(|b| b == button).unwrap();

    // Mark as pressed
    button.draw(state.lock().get_device(id).unwrap(), screen, true);

    // Run its logic
    match screen.id {
        1 => match button_id {
            0 => {
                let json = ha::get_state("weather.forecast_casa");
                let attributes = json["attributes"].clone();

                screen.clear_text(state.lock().get_device(id).unwrap());
                let text = format!("Temperature: {} C", attributes["temperature"].to_string())
                    .replace('"', "");
                screen.draw_text(state.lock().get_device(id).unwrap(), text);
            }

            _ => unreachable!(),
        },

        2 => match button_id {
            0 => {
                screen.clear_text(state.lock().get_device(id).unwrap());
                screen.draw_text(state.lock().get_device(id).unwrap(), "Test");
            }

            2 => {
                screen.clear_text(state.lock().get_device(id).unwrap());
                screen.draw_text(state.lock().get_device(id).unwrap(), "Message sent");

                tg::text("QMK -> XAP -> TauriClient -> Telegram");
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }

    gui::clear(state, id);
}
