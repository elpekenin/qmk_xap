/// Buttons are persistent elements on the screen, that change color while pressed
use crate::{
    user::{
        gui::{self, Screen, BG_COLOR, FG_COLOR, IMAGE_SIZE},
        UserData,
    },
    xap::hid::XAPDevice,
};
use std::fmt;
use xap_specs::protocol::ScreenPressed;

type HandlerFn = Box<dyn Fn(&XAPDevice, &Screen, &Button, &ScreenPressed, &UserData) + Send + 'static>;

pub struct Button {
    pub x: u16,
    pub y: u16,
    pub img: u8,
    pub handler: HandlerFn,
}

impl PartialEq for Button {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Debug for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Button | Coords: ({}, {}), img: {} |",
            self.x, self.y, self.img
        )
    }
}

impl Button {
    pub const TOLERANCE: u16 = 15;
    pub const SIZE: u16 = IMAGE_SIZE + 2 * Self::TOLERANCE;

    pub fn draw(&self, device: &XAPDevice, screen: &Screen, pressed: bool) {
        gui::draw::image_recolor(
            device,
            screen.id,
            self.x,
            self.y,
            self.img,
            if pressed { BG_COLOR } else { FG_COLOR },
            if pressed { FG_COLOR } else { BG_COLOR },
        );
    }

    pub fn handle(
        &self,
        device: &XAPDevice,
        screen: &Screen,
        msg: &ScreenPressed,
        user_data: &UserData,
    ) {
        // Mark as pressed
        self.draw(device, screen, true);

        // Run actual logic
        (self.handler)(device, screen, self, msg, user_data);

        // Mark as unpressed
        gui::clear(device, user_data);
    }
}
