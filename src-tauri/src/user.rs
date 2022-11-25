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

const BUTTON_0_X:  u16 = 180;
const BUTTON_0_Y:  u16 = 135;
const BUTTON_1_X:  u16 = 252;
const BUTTON_1_Y:  u16 = 135;
const BUTTON_SIZE: u16 = 48;
const TOLERANCE:   u16 = 60;

const HSV_WHITE: HSVColor = HSVColor{hue: 0, sat:0, val: 255};
const HSV_BLACK: HSVColor = HSVColor{hue: 0, sat:0, val: 0};


pub(crate) fn broadcast_callback(broadcast: BroadcastRaw, id: Uuid, state: &Arc<Mutex<XAPClient>>) {
    dotenv().ok();

    let msg: UserBroadcast = broadcast.into_xap_broadcast().unwrap();
    info!("Received {:?}", msg);

    // Clear UI
    let _ = state.lock().query(id, draw_button_request(BUTTON_0_X, BUTTON_0_Y, 0, false));
    let _ = state.lock().query(id, draw_button_request(BUTTON_1_X, BUTTON_1_Y, 1, false));

    if  BUTTON_0_X-TOLERANCE <= msg.x && msg.x <= BUTTON_0_X+BUTTON_SIZE+TOLERANCE
        && 
        BUTTON_0_Y-TOLERANCE <= msg.y && msg.y <= BUTTON_0_Y+BUTTON_SIZE+TOLERANCE {
        // Mark as pressed
        let _ = state.lock().query(id, draw_button_request(BUTTON_0_X, BUTTON_0_Y, 0, true));

        // Query HomeAssistant for current temperature
        let json = get_hasst_state("weather.forecast_casa");
        let attributes = json["attributes"].clone();

        // Format it and display on keyboard
        let _ = state.lock().query(id, clear_text());
        let text = format!("Temperature (HomeAssistant): {}C", attributes["temperature"].to_string()).replace('"', "");
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

        return;
    }

    if  BUTTON_1_X-TOLERANCE <= msg.x && msg.x <= BUTTON_1_X+BUTTON_SIZE+TOLERANCE
        &&
        BUTTON_1_Y-TOLERANCE <= msg.y && msg.y <= BUTTON_1_Y+BUTTON_SIZE+TOLERANCE {
        // Mark as pressed
        let _ = state.lock().query(id, draw_button_request(BUTTON_1_X, BUTTON_1_Y, 1, true));

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

        return;
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