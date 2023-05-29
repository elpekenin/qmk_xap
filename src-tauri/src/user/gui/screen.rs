/// Screens contain their own ID and elements on them
use crate::{
    user::{
        gui::{Button, Slider, SliderDirection},
        UserData,
    },
    xap::hid::XAPDevice,
};
use xap_specs::protocol::ScreenPressed;

#[derive(Debug)]
pub struct Screen {
    pub id: u8,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
}

impl Screen {
    fn get_button(&self, msg: &ScreenPressed) -> Option<&Button> {
        self.buttons.iter().find(|&button| {
            button.x - Button::TOLERANCE <= msg.x
                && msg.x <= button.x + Button::SIZE
                && button.y - Button::TOLERANCE <= msg.y
                && msg.y <= button.y + Button::SIZE
        })
    }

    fn get_slider(&self, msg: &ScreenPressed) -> Option<&Slider> {
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

    pub(crate) fn handle(&self, device: &XAPDevice, msg: &ScreenPressed, user_data: &UserData) {
        if msg.screen_id != self.id {
            return;
        }

        self.get_button(msg)
            .map_or((), |button| button.handle(device, self, msg, user_data));

        self.get_slider(msg).map_or((), |slider| {
            (slider.handler)(device, self, slider, msg, user_data)
        });
    }

    pub(crate) fn clear(&self, device: &XAPDevice) {
        for button in &self.buttons {
            button.draw(device, self, false);
        }

        for slider in &self.sliders {
            slider.clear(device, self);
        }
    }
}
