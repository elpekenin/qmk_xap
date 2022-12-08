use std::{sync::Arc, collections::HashMap};

use dotenv::dotenv;
use log::{info, error};
use once_cell::sync::Lazy;
use parking_lot::{Mutex};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::{
    header::{self, HeaderMap},
    self, Method,
};
use serde_json::{Value, Map};
use uuid::Uuid;
use xap_specs::protocol::{
    painter::*,
    BroadcastRaw, UserBroadcast
};

use crate::{xap::hid::{XAPClient,XAPDevice}};

// HTTP escape reserved characters
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'`').add(b'>').add(b'<').add(b'-');

// =====
// Buttons: Persistent element on the screen, that changes color while pressed
const TOLERANCE: u16 = 30;
const BUTTON_SIZE: u16 = IMAGE_SIZE + 2*TOLERANCE;
#[derive(Debug, Clone)]
struct Button {
    pub x: u16,
    pub y: u16,
    pub img: u8
}

// =====
// Sliders: Area where a variable takes different values depending on one of its coordinates, show a popup while pressed
#[derive(Debug, Clone)]
enum SliderDirection {
    Vertical,
    Horizontal
}
#[derive(Debug, Clone)]
struct Slider {
    pub direction: SliderDirection,
    pub start: u16,
    pub size: u16,
    pub x: u16,
    pub y: u16,
    pub img_map: HashMap<&'static str, u8>,
}
// =====
// Screen and contents info
#[derive(Debug, Clone)]
struct Screen {
    pub width: u16,
    pub height: u16,
    pub id: u8,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
}
static SCREENS: Lazy<Vec<Screen>> = Lazy::new(|| vec![
    // ILI9341
    Screen {
        width: 240,
        height: 320,
        id: 1,
        buttons: vec![
            Button {
                x: 50,
                y: 50,
                img: 0,
            }
        ],
        sliders: vec![],
    },

    // ILI9486
    Screen {
        width: 480,
        height: 320,
        id: 2,
        buttons: vec![
            Button {
                x: 150,
                y: 150,
                img: 6,
            }
        ],
        sliders: vec![
            Slider {
                direction: SliderDirection::Vertical,
                start: 400,
                size: 80,
                x: 240,
                y: 160,
                img_map: HashMap::from([
                    ("0", 0),
                    ("1", 1),
                    ("2", 2),
                    ("3", 3),
                    ("4", 4),
                    ("5", 5),
                ]),
            }
        ],
    }
]);

// Assets size
const IMAGE_SIZE: u16 = 48;

// Color palette
const HSV_WHITE: HSVColor = HSVColor{hue: 0, sat:0, val: 255};
const HSV_BLACK: HSVColor = HSVColor{hue: 0, sat:0, val: 0};
const BG_COLOR: HSVColor = HSV_BLACK;
const FG_COLOR: HSVColor = HSV_WHITE;

// Public functions
pub(crate)fn on_init() {
    // Make sure Home Assistant is running
    match std::process::Command::new("sh")
            .arg("-c")
            .arg("sudo systemctl start docker && cd $HOME/docker  && docker compose up -d")
            .output()
    {
        Ok(_) => info!("on_init went correctly"),
        Err(out) => error!("on_init failed due to: {out}")
    }
}


pub(crate) fn on_device_connection(device: &XAPDevice) {
    // Sleep is needed, so that screen is init'ed
    std::thread::sleep(std::time::Duration::from_millis(500));

    for screen in &*SCREENS {
        // Clear screen
        let _ = device.query(PainterDrawRect (
            PainterRect {
                dev: screen.id,
                left: 0,
                top: 0,
                right: screen.width,
                bottom: screen.height,
                color: BG_COLOR,
                filled: 1
            }
        ));

        // Show connection
        let _ = device.query(PainterDrawTextRecolor(
            PainterTextRecolor {
                dev: screen.id,
                x: 15,
                y: 15,
                font: 0,
                fg_color: FG_COLOR,
                bg_color: BG_COLOR,
                text: "Connected to Tauri".into(),
            }
        ));

        // Print buttons
        for button in &screen.buttons {
            let _ = device.query(PainterDrawImageRecolor (
                PainterImageRecolor {
                    dev: screen.id,
                    x: button.x,
                    y: button.y,
                    img: button.img,
                    fg_color: FG_COLOR,
                    bg_color: BG_COLOR,
                }
            ));
        }
    }
}


pub(crate) fn on_close(state: Arc<Mutex<XAPClient>>) {
    for device in state.clone().lock().get_devices() {
        for screen in &*SCREENS {
            // Clear screen
            let _ = device.query(PainterDrawRect (
                PainterRect {
                    dev: screen.id,
                    left: 0,
                    top: 0,
                    right: screen.width,
                    bottom: screen.height,
                    color: BG_COLOR,
                    filled: 1
                }
            ));

            // Show text
            let _ = device.query(PainterDrawTextRecolor(
                PainterTextRecolor {
                    dev: screen.id,
                    x: 15,
                    y: 15,
                    font: 0,
                    fg_color: FG_COLOR,
                    bg_color: BG_COLOR,
                    text: "Tauri app was closed".into(),
                }
            ));
        }
    }
}


pub(crate) fn broadcast_callback(broadcast: BroadcastRaw, id: Uuid, state: &Arc<Mutex<XAPClient>>) {
    // Read `.env` file
    dotenv().ok();

    // Parse raw data
    let msg: UserBroadcast = broadcast.into_xap_broadcast().unwrap();

    info!("Received {msg:?}");

    // Clear any leftover graphics
    clear_ui(id, state);

    // Run logic, code assumes that sliders and buttons don't overlap, if they do button will have preference
    match check_buttons(msg.clone()) {
        u8::MAX => {},
        button_id => {
            handle_button(msg.clone(), id, state, button_id);
        }
    }

    match check_sliders(msg.clone()) {
        u8::MAX => {},
        slider_id => {
            handle_slider(msg.clone(), id, state, slider_id);
        }
    }
}


// ------------------------------------------------ User's logic ------------------------------------------------
fn handle_button(msg: UserBroadcast, id: Uuid, state: &Arc<Mutex<XAPClient>>, button_id: u8) {
    let screen = get_screen_from_id(msg.dev).unwrap();

    // Mark as pressed
    let _ = state.lock().query(id, draw_button(screen.clone(), button_id, true));

    // Run its logic
    match get_screen_from_id(msg.dev) {
        None => {
            return;
        },

        Some(screen) => {
            match screen.id {
                1 => {
                    match button_id {
                        0 => {
                            // Query HomeAssistant for current temperature
                            let json = get_hasst_state("weather.forecast_casa");
                            let attributes = json["attributes"].clone();

                            // Format it and display on keyboard
                            let _ = state.lock().query(id, clear_text(screen.clone()));
                            let text = format!("Temperature (HomeAssistant): {}ºC", attributes["temperature"].to_string()).replace('"', "");
                            let _ = state.lock().query(id, draw_text(screen.clone(), text));
                        },
                        
                        v => error!("No logic for button {v}")
                    }
                },

                2 => {
                    match button_id {
                        0 => {
                            let _ = state.lock().query(id, clear_text(screen.clone()));
                            let _ = state.lock().query(id, draw_text(screen.clone(), get_pokeapi(msg.x+msg.y)));
                        },
                        
                        // 2 => {
                        //     // Show feedback
                        //     let _ = state.lock().query(id, clear_text(screen.clone()));
                        //     let _ = state.lock().query(id, draw_text(screen.clone(), "Message sent"));

                        //     // Send Telegram message
                        //     send_tg_msg("QMK -> XAP -> TauriClient -> Telegram");
                        // },

                        v => error!("No logic for button {v}")
                    }
                },

                v => error!("Invalid screen id: {v}")
            }
        }
    }

    clear_ui(id, state);
}

fn handle_slider(msg: UserBroadcast, id: Uuid, state: &Arc<Mutex<XAPClient>>, slider_id: u8) {
    match slider_id {
        0 => {
            let intensity = 5 - (msg.y * 6 / 321) as u16;
            let _ = state.lock().query(id, draw_slider(get_screen_from_id(msg.dev).unwrap().clone(), slider_id, intensity));
            set_light_intensity(intensity);
        },

        v => error!("No logic for slider {v}")
    }
}


// ------------------------------------------------ Event parsing helpers ------------------------------------------------
fn get_screen_from_id(id: u8) -> Option<&'static Screen> {
    for screen in &*SCREENS {
        if screen.id == id {
            return Some(screen);
        }
    }

    None
}

fn check_buttons(msg: UserBroadcast) -> u8 {
    match get_screen_from_id(msg.dev) {
        None => u8::MAX,

        // TODO equivalent of Python's `enumerate`?
        Some(screen) => {
            for i in 0..screen.buttons.len() {
                let button = &screen.buttons[i];

                if  button.x-TOLERANCE <= msg.x && msg.x <= button.x+BUTTON_SIZE
                    &&
                    button.y-TOLERANCE <= msg.y && msg.y <= button.y+BUTTON_SIZE {
                        return i as u8;
                }
            }

            u8::MAX
        }
    }
}

fn check_sliders(msg: UserBroadcast) -> u8 {
    match get_screen_from_id(msg.dev) {
        None => u8::MAX,

        Some(screen) => {
            for i in 0..screen.sliders.len() {
                let slider = &screen.sliders[i];

                match &slider.direction {
                    SliderDirection::Vertical => {
                        if  slider.start <= msg.x && msg.x <= slider.start + slider.size {
                            return i as u8;
                        }
                    }
                    SliderDirection::Horizontal => {
                        if  slider.start <= msg.y && msg.y <= slider.start + slider.size {
                            return i as u8;
                        }
                    }
                }
            }

            u8::MAX
        }
    }
}


// ------------------------------------------------ Drawing helpers ------------------------------------------------
fn draw_button(screen: Screen, id: impl Into<usize>, pressed: bool) -> PainterDrawImageRecolor {
    let button = &screen.buttons[id.into()];

    PainterDrawImageRecolor (
        PainterImageRecolor {
            dev: screen.id,
            x: button.x,
            y: button.y,
            img: button.img,
            fg_color: if pressed {BG_COLOR} else {FG_COLOR},
            bg_color: if pressed {FG_COLOR} else {BG_COLOR},
        }
    )
}


fn get_slider_image(slider: Slider, value: u16) -> u8 {
    match slider.img_map.get(&value.to_string() as &str) {
        Some(v) => *v,
        _ => u8::MAX
    }
}


fn draw_slider(screen: Screen, id: impl Into<usize>, value: u16) -> PainterDrawImage {
    let slider = &screen.sliders[id.into()];
    // Read value from Map, if can't be found defaults to `u8::MAX`
    let img = get_slider_image(slider.clone(), value);

    PainterDrawImage (
        PainterImage {
            dev: screen.id,
            x: slider.x,
            y: slider.y,
            // TODO: Better handling
            // This is somewhat unsafe, as custom QP_XAP code will try to read image array with offset bigger than its size
            // which is then passed to the QP function which detects it isn't a valid image an quits
            img
        }
    )
}

fn draw_text(screen: Screen, text: impl Into<Vec<u8>>) -> PainterDrawTextRecolor {
    PainterDrawTextRecolor (
        PainterTextRecolor {
            dev: screen.id,
            x: 0,
            y: screen.height-40,
            font:0,
            fg_color: FG_COLOR,
            bg_color: BG_COLOR,
            text: text.into()
        }
    )
}


// ------------------------------------------------ Cleaning helpers ------------------------------------------------
fn clear_ui(id: Uuid, state: &Arc<Mutex<XAPClient>>) {
    for screen in &*SCREENS {
        for j in 0..screen.buttons.len() {
            let _ = state.lock().query(id, draw_button(screen.clone(), j, false));
        }

        for j in 0..screen.sliders.len() {
            let _ = state.lock().query(id, clear_slider(screen.clone(), j));
        }
    }
}

fn clear_slider(screen: Screen, id: usize) -> PainterDrawRect {
    let slider = &screen.sliders[id];
    PainterDrawRect (
        PainterRect {
            dev: screen.id,
            left: slider.x,
            top: slider.y,
            right: slider.x+IMAGE_SIZE,
            bottom: slider.y+IMAGE_SIZE,
            color: BG_COLOR,
            filled: 1,
        }
    )
}

fn clear_text(screen: Screen) -> PainterDrawRect {
    PainterDrawRect(
        PainterRect{
            dev: screen.id,
            left: 0,
            top: screen.height-40,
            right: screen.width,
            bottom: screen.height,
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