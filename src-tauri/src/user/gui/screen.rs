/// Screens contain information about their geometry and elements on them
use crate::{
    user::{
        gui::{self, draw, Button, Slider, SliderDirection},
        handlers,
    },
    xap::hid::{XAPClient, XAPDevice},
};
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;
use xap_specs::protocol::UserBroadcast;

#[derive(Debug, Clone)]
pub struct Screen {
    pub id: u8,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
}

impl Screen {
    pub fn draw_text(&self, device: &XAPDevice, text: impl Into<Vec<u8>>) {
        let height = draw::geometry(device, self.id).height;
        draw::text_recolor(
            device,
            self.id,
            0,
            height - gui::FONT_SIZE,
            0,
            gui::FG_COLOR,
            gui::BG_COLOR,
            text,
        );
    }

    pub fn clear_text(&self, device: &XAPDevice) {
        let geometry = draw::geometry(device, self.id);
        draw::rect(
            device,
            self.id,
            0,
            geometry.height - gui::FONT_SIZE,
            geometry.width,
            geometry.height,
            gui::BG_COLOR,
            true,
        );
    }

    fn get_button(&self, msg: &UserBroadcast) -> Option<&Button> {
        self.buttons.iter().find(|&button| {
            button.x - Button::TOLERANCE <= msg.x
                && msg.x <= button.x + Button::SIZE
                && button.y - Button::TOLERANCE <= msg.y
                && msg.y <= button.y + Button::SIZE
        })
    }

    fn get_slider(&self, msg: &UserBroadcast) -> Option<&Slider> {
        for slider in &self.sliders {
            match &slider.direction {
                SliderDirection::Vertical => {
                    if slider.start <= msg.x && msg.x <= slider.start + slider.size {
                        return Some(slider);
                    }
                }
                SliderDirection::Horizontal => {
                    if slider.start <= msg.y && msg.y <= slider.start + slider.size {
                        return Some(slider);
                    }
                }
            }
        }
        None
    }

    pub(crate) fn handle(&self, state: &Arc<Mutex<XAPClient>>, id: &Uuid, msg: &UserBroadcast) {
        if msg.screen_id != self.id {
            return;
        }

        self.get_button(msg)
            .map_or((), |button| handlers::button(state, id, self, button));

        self.get_slider(msg).map_or((), |slider| {
            let coord = match slider.direction {
                SliderDirection::Vertical => msg.y,
                SliderDirection::Horizontal => msg.x,
            };

            handlers::slider(state, id, self, slider, coord);
        });
    }
}
