use crate::{
    user::{
        gui::{self, HSV_BLACK},
        os, UserData,
    },
    xap::hid::XAPDevice,
};
use chrono::{self, Timelike};

use log::info;
use reqwest::Method;

const SCREEN_ID: u8 = 1;
pub const IMG_SIZE: u16 = 24;
pub const X: u16 = 15;
pub const Y: u16 = os::FONT_SIZE + os::Y;

const LATITUDE: f64 = 37.60;
const LONGITUDE: f64 = -0.97;

fn get_forecast() -> Option<u8> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={:.2}&longitude={:.2}&hourly=weathercode",
        LATITUDE, LONGITUDE
    );

    let response = super::request(Method::GET, url, None, None)?;

    let hour = chrono::offset::Utc::now().hour() as usize;

    Some(response[0].get("hourly")?.get("weathercode")?.as_array()?[hour].as_u64()? as u8)
}

fn forecast_to_img_id(forecast: Option<u8>) -> Option<u8> {
    match forecast? {
        0 => Some(9),       // clear (sunny)
        1..=3 => Some(7),   // cloudy
        61..=65 => Some(8), // rain
        80..=82 => Some(8), // rain
        id => {
            // anything else
            info!("No img for forecast with id: {id}");
            None
        }
    }
}

pub fn draw(device: &XAPDevice, _user_data: &mut UserData) {
    let img = forecast_to_img_id(get_forecast());

    match img {
        Some(img) => gui::draw::image(device, SCREEN_ID, X, Y, img),
        None => gui::draw::rect(
            device,
            SCREEN_ID,
            X,
            Y,
            X + IMG_SIZE,
            Y + IMG_SIZE,
            HSV_BLACK,
            true,
        ),
    }
}
