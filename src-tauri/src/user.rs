use std::sync::Arc;

use dotenv::dotenv;
use log::info;
use parking_lot::{Mutex};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest;
use serde_json::{Value, Map};
use uuid::Uuid;
use xap_specs::protocol::{
    painter::*,
    BroadcastRaw, UserBroadcast
};

use crate::xap::hid::XAPClient;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'`').add(b'>').add(b'<').add(b'-');

const N_BUTTONS: usize = 2;
const BUTTONS_X: [u16; N_BUTTONS] = [180, 252];
const BUTTONS_Y: [u16; N_BUTTONS] = [135, 135];
const BUTTON_SIZE: u16 = 48;
const TOLERANCE: u16 = 60;

const HSV_WHITE: HSVColor = HSVColor{hue: 0, sat:0, val: 255};
const HSV_BLACK: HSVColor = HSVColor{hue: 0, sat:0, val: 0};


pub(crate) fn broadcast_callback(broadcast: BroadcastRaw, id: Uuid, state: &Arc<Mutex<XAPClient>>) {
    dotenv().ok();

    let msg: UserBroadcast = broadcast.into_xap_broadcast().unwrap();

    // Clear buttons
    for i in 0..N_BUTTONS {
        let _ = state.lock().query(id, draw_button_request(BUTTONS_X[i], BUTTONS_Y[i], i as u8, false));
    };

    // Currently pressed botton
    let button = get_button(msg);

    // Mark it as pressed
    if button > -1 {
        let _ = state.lock().query(
            id,
            draw_button_request(
                BUTTONS_X[button as usize],
                BUTTONS_Y[button as usize],
                button as u8,
                true
            )
        );
    }

    // Buttons' logic
    match button { 
        0 => {
            // Query HomeAssistant for current temperature
            let json = get_hasst_state("weather.forecast_casa");
            let attributes = json["attributes"].clone();

            // Format it and display on keyboard
            let _ = state.lock().query(id, clear_text());
            let text = format!("Temperature (HomeAssistant): {}ºC", attributes["temperature"].to_string()).replace('"', "");
            let request = PainterDrawText(
                PainterText {
                    dev:0,
                    x: 120,
                    y: 240,
                    font:0,
                    text: text.clone().into_bytes()
                }
            );
            let _ = state.lock().query(id, request);
        },

        1 => {
            // Show feedback
            let _ = state.lock().query(id, clear_text());
            let request = PainterDrawText(
                PainterText {
                    dev:0,
                    x: 220,
                    y: 240,
                    font:0,
                    text: "Message sent".to_string().into_bytes()
                }
            );
            let _ = state.lock().query(id, request);

            // Send Telegram message
            send_tg_msg("QMK -> XAP -> TauriClient -> Telegram");
        },

        _ => {}
    }
}


fn clear_text() -> PainterDrawRect {
    PainterDrawRect(
        PainterRect{
            dev: 0,
            left: 0,
            top: 235,
            right: 480,
            bottom: 320,
            color: HSV_BLACK,
            filled: 1,
        }
    )
}

fn draw_button_request(button_x: u16, button_y: u16, img_id: u8, pressed: bool) -> PainterDrawImageRecolor {
    PainterDrawImageRecolor(
        PainterImageRecolor {
            dev: 0,
            x: button_x,
            y: button_y,
            img: img_id,
            fg_color: if pressed {HSV_BLACK} else {HSV_WHITE},
            bg_color: if pressed {HSV_WHITE} else {HSV_BLACK},
        }
    )
}

fn get_button(msg: UserBroadcast) ->i8 {
    for i in 0..N_BUTTONS {
        if  BUTTONS_X[i]-TOLERANCE <= msg.x && msg.x <= BUTTONS_X[i]+BUTTON_SIZE+TOLERANCE
            &&
            BUTTONS_Y[i]-TOLERANCE <= msg.y && msg.y <= BUTTONS_Y[i]+BUTTON_SIZE+TOLERANCE {
                info!("Button {i} was pressed");
                // Mark as pressed
                return i as i8;
        }
    }
    -1
}

fn get_hasst_state(entity_id: impl Into<String>) -> Map<String, Value> {
    let entity_id = entity_id.into();

    let hasst_token = std::env::var("HASST_TOKEN").unwrap();
    let hasst_base_url = std::env::var("HASST_BASE_URL").unwrap_or("http://localhost:8123".to_string());
    let hasst_url = format!("{}/api/states/{}", hasst_base_url, entity_id);
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(hasst_url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", hasst_token))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .expect("Couldn't make the petition");

    let content = if response.status() == reqwest::StatusCode::OK {
        response.text().unwrap()
    } else {
        info!("HASST [{}], {}", response.status(), response.text().unwrap());
        "".to_string()
    };

   serde_json::from_str(&content).unwrap()
}

fn send_tg_msg(text: impl Into<String>) {
    let text = text.into();

    let tg_token = std::env::var("TG_TOKEN").unwrap();
    let tg_id = std::env::var("TG_ID").unwrap();
    let method = "sendMessage";
    let client = reqwest::blocking::Client::new();

    let escaped = utf8_percent_encode(&text, FRAGMENT).to_string();
    let tg_url = format!("https://api.telegram.org/bot{}/{}?chat_id={}&text={}", tg_token, method, tg_id, escaped);
    let _ = client.get(tg_url).send().unwrap();
}