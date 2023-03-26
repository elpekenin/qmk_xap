use crate::{
    user::{
        gui::{self, HSV_BLACK, FONT_SIZE},
        http::FRAGMENT,
        UserData,
    },
    xap::hid::XAPDevice,
};
use chrono::{self, Timelike};

use log::info;
use percent_encoding::utf8_percent_encode;
use reqwest::Method;

use super::home_assistant;

const SCREEN_ID: u8 = 1;
const IMG_SIZE: u16 = 24;

fn get_forecast() -> u8 {
    let latitude = 37.60;
    let longitude = -0.97;
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={:.2}&longitude={:.2}&hourly=weathercode",
        latitude, longitude
    );

    let response = super::request(Method::GET, url, None, None).unwrap();

    let hour = chrono::offset::Utc::now().hour() as usize;

    let forecast = response
        .get("hourly")
        .unwrap()
        .get("weathercode")
        .unwrap()
        .as_array()
        .unwrap()[hour]
        .as_u64()
        .unwrap();

    forecast as u8
}

fn forecast_to_img_id(forecast: u8) -> Option<u8> {
    let temp = match forecast {
        0 => 8,       // clear (sunny)
        1..=3 => 6,   // cloudy
        61..=65 => 7, // rain
        80..=82 => 7, // rain
        _ => u8::MAX, // anything else
    };

    if temp == u8::MAX {
        info!("No img for forecast with id: {forecast}");
        return None;
    }

    Some(temp)
}

pub fn draw(device: &XAPDevice, _user_data: &mut UserData) {
    let img = forecast_to_img_id(get_forecast());

    let width = gui::draw::geometry(device, SCREEN_ID).width;
    let x = width - IMG_SIZE;
    let y = FONT_SIZE;

    match img {
        Some(img) => gui::draw::image(device, SCREEN_ID, x, y, img),
        None => gui::draw::rect(device, SCREEN_ID, x, y, x + IMG_SIZE, y + IMG_SIZE, HSV_BLACK, true),
    }
}
