use crate::{
    user::{
        gui::{self, HSV_BLACK},
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

    let screen_id = 1;
    let geometry = gui::draw::geometry(device, screen_id);
    let width = geometry.width;
    let height = geometry.height;
    let img_size = 24;

    let x = width - img_size;
    let y = height - img_size;

    match img {
        Some(img) => gui::draw::image(device, screen_id, x, y, img),
        None => gui::draw::rect(device, screen_id, x, y, width, height, HSV_BLACK, true),
    }
}
