/// Sliders are areas where a variable takes different values depending on one coord, and show a popup while pressed
use crate::{
    user::gui::{draw, Screen, BG_COLOR, IMAGE_SIZE},
    xap::hid::XAPDevice,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SliderDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slider {
    pub direction: SliderDirection,
    pub start: u16,
    pub size: u16,
    pub x: u16,
    pub y: u16,
    pub img_map: HashMap<&'static str, u8>,
}

impl Slider {
    fn get_image(&self, value: u16) -> u8 {
        match self.img_map.get(&value.to_string() as &str) {
            Some(v) => *v,
            _ => u8::MAX,
        }
    }

    pub fn draw(&self, device: &XAPDevice, screen: &Screen, value: u16) {
        // Read value from Map, if can't be found defaults to `u8::MAX`
        let img = self.get_image(value);

        draw::image(device, screen.id, self.x, self.y, img);
    }

    pub fn clear(&self, device: &XAPDevice, screen: &Screen) {
        draw::rect(
            device,
            screen.id,
            self.x,
            self.y,
            self.x + IMAGE_SIZE,
            self.y + IMAGE_SIZE,
            BG_COLOR,
            true,
        );
    }
}
