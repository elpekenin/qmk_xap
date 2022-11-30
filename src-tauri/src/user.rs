use std::{sync::Arc, collections::HashMap};

use dotenv::dotenv;
use log::{info, error};
use parking_lot::{Mutex};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::{
    header::{self, HeaderMap},
    self, Method,
};
use serde_json::{Value, Map};
use tauri::utils::assets::phf::phf_map;
use uuid::Uuid;
use xap_specs::protocol::{
    painter::*,
    BroadcastRaw, UserBroadcast
};

use crate::{xap::hid::{XAPClient,XAPDevice}};

// HTTP escape reserved characters
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'`').add(b'>').add(b'<').add(b'-');

// Display info
const SCREEN_WIDTH: u16 = 480;
const SCREEN_HEIGHT: u16 = 320;
const SCREEN_ID: u8 = 0;

// Assets size
const IMAGE_SIZE: u16 = 48;

// Color palette
const HSV_WHITE: HSVColor = HSVColor{hue: 0, sat:0, val: 255};
const HSV_BLACK: HSVColor = HSVColor{hue: 0, sat:0, val: 0};
const BG_COLOR: HSVColor = HSV_BLACK;
const FG_COLOR: HSVColor = HSV_WHITE;

// =====
// Buttons: Persistent element on the screen, that changes color while pressed
const N_BUTTONS: usize = 1;
//Size and position of each one
const BUTTONS_X: [u16; N_BUTTONS] = [150];
const BUTTONS_Y: [u16; N_BUTTONS] = [150];
//Extra hitbox around each one
const TOLERANCE: u16 = 30;
const BUTTON_SIZE: u16 = IMAGE_SIZE + 2*TOLERANCE;
//Button to image_id mapping
const BUTTON2IMG: [u8; N_BUTTONS] = [6];

// =====
// Sliders: Area where a variable takes different values depending on one of its coordinates, show a popup while pressed
const N_SLIDERS: usize = 1;
//Size and position of each one
const SLIDERS_X: [u16; N_SLIDERS] = [430];
const SLIDERS_Y: [u16; N_SLIDERS] = [0];
const SLIDERS_SIZE_X: [u16; N_SLIDERS] = [50];
const SLIDERS_SIZE_Y: [u16; N_SLIDERS] = [320];
//Position where the popups are shown
const SLIDERS_POPUP_X: [u16; N_SLIDERS] = [SCREEN_WIDTH/2];
const SLIDERS_POPUP_Y: [u16; N_SLIDERS] = [SCREEN_HEIGHT/2];
//Slider value to image_id mapping
type SliderMap = phf::Map<&'static str, u8>;
//Light brightness (0-5) slider
const _SLIDER_MAP_0: SliderMap = phf_map! {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5
};
const SLIDER2IMG: [SliderMap; N_SLIDERS] = [
    _SLIDER_MAP_0
];

// =====
// Text: A region is reserved for the text, taking all the width of the screen within the vertical range
const TEXT_START: u16 = 300;
const TEXT_END: u16 = SCREEN_HEIGHT;

// Public functions
pub(crate)fn on_init() {
    match std::process::Command::new("sh")
            .arg("-c")
            .arg("sudo systemctl start docker && cd $HOME/docker  && docker compose up -d")
            .output()
    {
        Ok(_) => error!("on_init went correctly"),
        Err(out) => error!("on_init failed due to: {out}")
    }
}


pub(crate) fn on_device_connection(device: &XAPDevice) {
    // Sleep is needed, so that screen is init'ed
    std::thread::sleep(std::time::Duration::from_millis(2000));

    // Show connection
    let _ = device.query(PainterDrawTextRecolor(
        PainterTextRecolor {
            dev: SCREEN_ID,
            x: 15,
            y: 15,
            font: 0,
            fg_color: FG_COLOR,
            bg_color: BG_COLOR,
            text: "Connected to Tauri".into(),
        }
    ));

    // Print buttons
    for id in 0..N_BUTTONS {
        let _ = device.query(PainterDrawImageRecolor (
            PainterImageRecolor {
                dev: SCREEN_ID,
                x: BUTTONS_X[id],
                y: BUTTONS_Y[id],
                img: BUTTON2IMG[id],
                fg_color: FG_COLOR,
                bg_color: BG_COLOR,
            }
        ));
    }
}


pub(crate) fn on_close(state: Arc<Mutex<XAPClient>>) {
    for device in state.clone().lock().get_devices() {
        // Clear screen
        let _ = device.query(PainterDrawRect (
            PainterRect {
                dev: SCREEN_ID,
                left: 0,
                top: 0,
                right: SCREEN_WIDTH,
                bottom: SCREEN_HEIGHT,
                color: BG_COLOR,
                filled: 1
            }
        ));

        // Show text
        let _ = device.query(PainterDrawTextRecolor(
            PainterTextRecolor {
                dev: SCREEN_ID,
                x: 15,
                y: 15,
                font: 0,
                fg_color: FG_COLOR,
                bg_color: BG_COLOR,
                text: "Tauri app was closed".into(),
            }
        ));
    };
}


pub(crate) fn broadcast_callback(broadcast: BroadcastRaw, id: Uuid, state: &Arc<Mutex<XAPClient>>) {
    // Read `.env` file
    dotenv().ok();

    // Parse raw data
    let msg: UserBroadcast = broadcast.into_xap_broadcast().unwrap();

    // Clear any leftover graphics
    clear_ui(id, state);

    // Run logic, code assumes that sliders and buttons don't overlap, if they do button will have preference
    match check_buttons(&msg) {
        u8::MAX => {},
        button_id => {
            handle_button(msg, id, state, button_id);
            return;
        }
    }

    match check_sliders(&msg) {
        u8::MAX => {},
        slider_id => {
            handle_slider(msg, id, state, slider_id);
            return;
        }
    }
}


// ------------------------------------------------ User's logic ------------------------------------------------
fn handle_button(msg: UserBroadcast, id: Uuid, state: &Arc<Mutex<XAPClient>>, button_id: u8) {
    if button_id == u8::MAX {
        return;
    }

    // Mark as pressed
    let _ = state.lock().query(id, draw_button(button_id, true));

    // Run its logic
    match button_id {
        0 => {
            let _ = state.lock().query(id, clear_text());
            let _ = state.lock().query(id, draw_text(get_pokeapi(msg.x+msg.y)));
        },

        1 => {
            // Query HomeAssistant for current temperature
            let json = get_hasst_state("weather.forecast_casa");
            let attributes = json["attributes"].clone();

            // Format it and display on keyboard
            let _ = state.lock().query(id, clear_text());
            let text = format!("Temperature (HomeAssistant): {}ºC", attributes["temperature"].to_string()).replace('"', "");
            let _ = state.lock().query(id, draw_text(text));
        },

        2 => {
            // Show feedback
            let _ = state.lock().query(id, clear_text());
            let _ = state.lock().query(id, draw_text("Message sent"));

            // Send Telegram message
            send_tg_msg("QMK -> XAP -> TauriClient -> Telegram");
        },

        v => error!("No logic for button {v}")
    }

    clear_ui(id, state);
}

fn handle_slider(msg: UserBroadcast, id: Uuid, state: &Arc<Mutex<XAPClient>>, slider_id: u8) {
    if slider_id == u8::MAX {
        return;
    }

    match slider_id {
        0 => {
            let intensity = 5 - (msg.y * 6 / 321) as u16;
            let _ = state.lock().query(id, draw_slider(slider_id, intensity));
            set_light_intensity(intensity);
        },

        v => error!("No logic for slider {v}")
    }
}


// ------------------------------------------------ Event parsing helpers ------------------------------------------------
fn check_buttons(msg: &UserBroadcast) -> u8 {
    for i in 0..N_BUTTONS {
        if  BUTTONS_X[i]-TOLERANCE <= msg.x && msg.x <= BUTTONS_X[i]+BUTTON_SIZE
            &&
            BUTTONS_Y[i]-TOLERANCE <= msg.y && msg.y <= BUTTONS_X[i]+BUTTON_SIZE {
                return i as u8;
        }
    }

    u8::MAX
}

fn check_sliders(msg: &UserBroadcast) -> u8 {
    for i in 0..N_SLIDERS {
        if  SLIDERS_X[i] <= msg.x && msg.x <= SLIDERS_X[i]+SLIDERS_SIZE_X[i]
            &&
            SLIDERS_Y[i] <= msg.y && msg.y <= SLIDERS_Y[i]+SLIDERS_SIZE_Y[i] {
                return i as u8;
        }
    }

    u8::MAX
}


// ------------------------------------------------ Drawing helpers ------------------------------------------------
fn draw_button(id: impl Into<usize>, pressed: bool) -> PainterDrawImageRecolor {
    let id = id.into();

    PainterDrawImageRecolor (
        PainterImageRecolor {
            dev: SCREEN_ID,
            x: BUTTONS_X[id],
            y: BUTTONS_Y[id],
            img: BUTTON2IMG[id],
            fg_color: if pressed {BG_COLOR} else {FG_COLOR},
            bg_color: if pressed {FG_COLOR} else {BG_COLOR},
        }
    )
}


fn get_slider_image(id: usize, value: u16) -> u8 {
    match SLIDER2IMG[id].get(&value.to_string() as &str) {
        Some(v) => *v,
        _ => u8::MAX
    }
}


fn draw_slider(id: impl Into<usize>, value: u16) -> PainterDrawImage {
    let id = id.into();
    // Read value from Map, if can't be found defaults to `u8::MAX`
    let img = get_slider_image(id, value);

    PainterDrawImage (
        PainterImage {
            dev: SCREEN_ID,
            x: SLIDERS_POPUP_X[id],
            y: SLIDERS_POPUP_Y[id],
            // TODO: Better handling
            // This is somewhat unsafe, as custom QP_XAP code will try to read image array with offset bigger than its size
            // which is then passed to the QP function which detects it isn't a valid image an quits
            img
        }
    )
}

fn draw_text(text: impl Into<Vec<u8>>) -> PainterDrawTextRecolor {
    PainterDrawTextRecolor (
        PainterTextRecolor {
            dev: SCREEN_ID,
            x: 120,
            y: TEXT_START,
            font:0,
            fg_color: FG_COLOR,
            bg_color: BG_COLOR,
            text: text.into()
        }
    )
}


// ------------------------------------------------ Cleaning helpers ------------------------------------------------
fn clear_ui(id: Uuid, state: &Arc<Mutex<XAPClient>>) {
    for i in 0..N_BUTTONS {
        let _ = state.lock().query(id, draw_button(i, false));
    }

    for i in 0..N_SLIDERS {
        let _ = state.lock().query(id, clear_slider(i));
    }

    // let _ = state.lock().query(id, clear_text());
}

fn clear_slider(id: usize) -> PainterDrawRect {
    PainterDrawRect (
        PainterRect {
            dev: SCREEN_ID,
            left: SLIDERS_POPUP_X[id],
            top: SLIDERS_POPUP_Y[id],
            right: SLIDERS_POPUP_X[id]+IMAGE_SIZE,
            bottom: SLIDERS_POPUP_Y[id]+IMAGE_SIZE,
            color: BG_COLOR,
            filled: 1,
        }
    )
}

fn clear_text() -> PainterDrawRect {
    PainterDrawRect(
        PainterRect{
            dev: SCREEN_ID,
            left: 0,
            top: TEXT_START,
            right: SCREEN_WIDTH,
            bottom: TEXT_END,
            color: BG_COLOR,
            filled: 1,
        }
    )
}


// ------------------------------------------------ HTTP helper ------------------------------------------------
fn http_request(method: Method, url: String, headers: Option<HeaderMap>, payload: Option<HashMap<&str, String>>) -> Option<Map<String, Value>> {
    let _client = reqwest::blocking::Client::new();

    let mut client = match method {
        Method::GET => {
            _client
                .get(&url)
        },

        Method::POST => {
            if payload == None {
                error!("Tried to POST without payload");
                return None;
            }

            _client
                .post(&url)
                .json(&payload)
        },
        
        _ => {
            error!("Unsupported HTTP method");
            return None;
        }
    };

    if headers != None {
        client = client.headers(headers.unwrap());
    }

    let response = match client.send() {
        Ok(r) => r,
        Err(_) => {
            error!("Couldn't make a request to {url}");
            return None;
        }
    };

    let text = match response.text() {
        Ok(t) => t,
        Err(_) => {
            error!("Couldn't read text from response");
            return None;
        }
    };
    
    // let status_code = response.status();
    // if  status_code != reqwest::StatusCode::OK {
    //     info!("[{status_code}] - {text}");
    //     return None;
    // };
    
    match serde_json::from_str(&text) {
        Ok(r) => r,
        _ => None
    }
}


// ------------------------------------------------ HASST ------------------------------------------------
fn get_hasst_state(entity_id: impl Into<String>) -> Map<String, Value> {
    let entity_id = entity_id.into();

    let hasst_token = std::env::var("HASST_TOKEN").unwrap();
    let hasst_base_url = std::env::var("HASST_BASE_URL").unwrap_or("http://localhost:8123".to_string());
    let url = format!("{}/api/states/{}", hasst_base_url, entity_id);

    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, format!("Bearer {}", hasst_token).parse().unwrap());

    http_request(Method::GET, url, Some(headers), None).unwrap()
}

fn set_light_intensity(level: u16) {
    let level = 255/5 * level;

    let hasst_token = std::env::var("HASST_TOKEN").unwrap();
    let hasst_base_url = std::env::var("HASST_BASE_URL").unwrap_or("http://localhost:8123".to_string());
    let lightbulb = std::env::var("LIGHTBULB_ID").unwrap();

    let url = format!("{}/api/services/light/turn_on", hasst_base_url);

    let mut payload = HashMap::new();
    payload.insert("entity_id", lightbulb);
    payload.insert("brightness", level.to_string());

    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, format!("Bearer {hasst_token}").parse().unwrap());

    http_request(Method::POST, url, Some(headers), Some(payload));
}


// ------------------------------------------------ POKEAPI ------------------------------------------------
fn get_pokeapi(pokedex: impl Into<u16>) -> String {
    let pokedex = pokedex.into();

    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokedex);

    match http_request(Method::GET, url, None, None) {
        Some(r) => r.get("name").unwrap().to_string(),
        _ => "".to_string()
    }
}


// ------------------------------------------------ TELEGRAM ------------------------------------------------
fn send_tg_msg(text: impl Into<String>) {
    let text = text.into();

    let tg_token = std::env::var("TG_TOKEN").unwrap();
    let tg_id = std::env::var("TG_ID").unwrap();

    let escaped_text = utf8_percent_encode(&text, FRAGMENT).to_string();
    let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}", tg_token, tg_id, escaped_text);

    http_request(Method::GET, url, None, None);
}