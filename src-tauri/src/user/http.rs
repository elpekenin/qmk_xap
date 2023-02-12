pub mod home_assistant;
pub mod telegram;

use log::error;
use percent_encoding::{AsciiSet, CONTROLS};
use reqwest::{self, header::HeaderMap, Method};
use serde_json::{Map, Value};
use std::collections::HashMap;

// HTTP escape reserved characters
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'`')
    .add(b'>')
    .add(b'<')
    .add(b'-');

fn http_request(
    method: Method,
    url: String,
    headers: Option<HeaderMap>,
    payload: Option<HashMap<&str, String>>,
) -> Option<Map<String, Value>> {
    let _client = reqwest::blocking::Client::new();

    let mut client = match method {
        Method::GET => _client.get(&url),

        Method::POST => {
            if payload == None {
                error!("Tried to POST without payload");
                return None;
            }

            _client.post(&url).json(&payload)
        }

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
        _ => None,
    }
}
