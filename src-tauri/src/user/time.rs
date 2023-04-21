use chrono::prelude::*;
use chrono::Local;
use crate::{
    user::gui::{self, HSV_BLACK, HSV_WHITE},
    xap::hid::XAPDevice,
    UserData,
};

const SCREEN_ID: u8 = 1;
const X: u16 = 0;
const Y: u16 = 0;
const FONT: u8 = 0;

pub fn show(device: &XAPDevice, user_data: &mut UserData) {
    let now = Local::now();
    
    // Early stopping if minute hasn't changed
    let minute = now.minute();
    if minute == user_data.time.minute() {
        return;
    }

    // Update time
    user_data.time = now;

    // Draw it
    let day = now.day();
    let month = now.month();
    let hour = now.hour();
    let text = format!("{day}/{month} - {hour:02}:{minute:02}");
    gui::draw::text_recolor(device, SCREEN_ID, X, Y, FONT, HSV_WHITE, HSV_BLACK, text);
}