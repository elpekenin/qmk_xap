use crate::{
    user::{
        gui::{self, Button, Screen, Slider},
        http::home_assistant as ha,
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
        1 => {
            match button_id {
                0 => {
                    // Query HomeAssistant for current temperature
                    let json = ha::get_state("weather.forecast_casa");
                    let attributes = json["attributes"].clone();

                    // Format it and display on keyboard
                    screen.clear_text(state.lock().get_device(id).unwrap());
                    let text = format!("Temperature: {} C", attributes["temperature"].to_string())
                        .replace('"', "");
                    screen.draw_text(state.lock().get_device(id).unwrap(), text);
                }

                _ => unreachable!(),
            }
        }

        2 => {
            match button_id {
                0 => {
                    screen.clear_text(state.lock().get_device(id).unwrap());
                    screen.draw_text(state.lock().get_device(id).unwrap(), "Test");
                }

                // 2 => {
                //     // Show feedback
                //     let _ = state.lock().query(id, clear_text(screen.clone()));
                //     let _ = state.lock().query(id, draw_text(screen.clone(), "Message sent"));

                //     // Send Telegram message
                //     send_tg_msg("QMK -> XAP -> TauriClient -> Telegram");
                // },
                _ => unreachable!(),
            }
        }

        _ => unreachable!(),
    }

    gui::clear(state, id);
}
