/// Sliders are areas where a variable takes different values depending on one coord, and show a popup while pressed
use crate::{
    user::{
        gui::{draw, Screen, BG_COLOR, IMAGE_SIZE},
        UserData,
    },
    xap::hid::XAPDevice,
};
use log::info;
use std::{collections::HashMap, fmt};
use xap_specs::protocol::ScreenPressed;

#[derive(Debug, Clone, PartialEq)]
pub enum SliderDirection {
    Vertical,
    Horizontal,
}

type SliderFn =
    Box<dyn Fn(&XAPDevice, &Screen, &Slider, &ScreenPressed, &UserData) + Send + 'static>;

pub struct Slider {
    pub direction: SliderDirection,
    pub start: u16,
    pub size: u16,
    pub x: u16,
    pub y: u16,
    pub img_map: HashMap<&'static str, u8>,
    pub handler: SliderFn,
}

impl PartialEq for Slider {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.size == other.size && self.direction == other.direction
    }
}

impl fmt::Debug for Slider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Slider | Coords: ({}, {}), direction: {:?}, size: {}, imgs: {:#?} |",
            self.x, self.y, self.direction, self.size, self.img_map
        )
    }
}

impl Slider {
    fn get_image(&self, value: u16) -> Option<&u8> {
        self.img_map.get(&value.to_string() as &str)
    }

    pub fn coord(&self, msg: &ScreenPressed) -> u16 {
        match self.direction {
            SliderDirection::Vertical => msg.y,
            SliderDirection::Horizontal => msg.x,
        }
    }

    pub fn draw(&self, device: &XAPDevice, screen: &Screen, value: u16) {
        self.get_image(value).map_or_else(
            || info!("No image for value {value} in slider {:?}", self),
            |&img| draw::image(device, screen.id, self.x, self.y, img),
        )
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
