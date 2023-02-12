/// Buttons are persistent elements on the screen, that change color while pressed
use crate::{
    user::gui::{draw, Screen, BG_COLOR, FG_COLOR, IMAGE_SIZE},
    xap::hid::XAPDevice,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub x: u16,
    pub y: u16,
    pub img: u8,
}

impl Button {
    pub const TOLERANCE: u16 = 30;
    pub const SIZE: u16 = IMAGE_SIZE + 2 * Button::TOLERANCE;

    pub fn draw(&self, device: &XAPDevice, screen: &Screen, pressed: bool) {
        draw::image_recolor(
            device,
            screen.id,
            self.x,
            self.y,
            self.img,
            if pressed { BG_COLOR } else { FG_COLOR },
            if pressed { FG_COLOR } else { BG_COLOR },
        );
    }
}
