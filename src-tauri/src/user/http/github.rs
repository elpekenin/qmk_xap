use crate::{
    user::{self, gui, UserData},
    xap::hid::XAPDevice,
};

use anyhow::Ok;
use reqwest::{
    header::{self, HeaderMap},
    Method,
};

fn headers() -> Result<HeaderMap, anyhow::Error> {
    let mut headers = HeaderMap::new();

    headers.insert(header::ACCEPT, "application/vnd.github+json".parse()?);
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {}", user::get_var("GITHUB_TOKEN")).parse()?,
    );
    headers.insert(header::USER_AGENT, "QMK XAP".parse()?);

    Ok(headers)
}

fn get_notifications() -> u8 {
    // just the count of notifcations so far
    let url = "https://api.github.com/notifications";

    let headers = match headers() {
        std::result::Result::Ok(h) => h,
        Err(e) => {
            log::error!("{e}, trying to create headers");
            return 0;
        }
    };

    super::request(Method::GET, url, Some(headers), None).map_or(0, |r| r.len() as u8)
}

const SCREEN_ID: u8 = 1;
const IMG: u8 = 0;
const IMG_SIZE: u16 = 50;
const FONT: u8 = 1;
const FONT_SIZE: u16 = gui::FONT_SIZES[FONT as usize];
// hardcoded from 9341's size
const X: u16 = 320 - IMG_SIZE - 10;
const Y: u16 = user::time::Y;

pub fn draw(device: &XAPDevice, user_data: &mut UserData) {
    let notifications = get_notifications();

    if notifications == user_data.notifications {
        return;
    }

    user_data.notifications = notifications;
    if notifications == 0 {
        gui::draw::rect(
            device,
            SCREEN_ID,
            X,
            Y,
            X + IMG_SIZE,
            Y + IMG_SIZE,
            gui::BG_COLOR,
            true,
        );
        return;
    }

    gui::draw::image(device, SCREEN_ID, X, Y, IMG);
    gui::draw::text_recolor(
        device,
        SCREEN_ID,
        X + IMG_SIZE - FONT_SIZE,
        Y,
        FONT,
        gui::HSV_WHITE,
        gui::HSV_RED,
        format!("{}", notifications),
    )
}
