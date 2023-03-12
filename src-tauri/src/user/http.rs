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

fn request(
    method: Method,
    url: impl Into<String>,
    headers: Option<HeaderMap>,
    payload: Option<HashMap<&str, String>>,
) -> Option<Map<String, Value>> {
    let client = reqwest::blocking::Client::new();

    let url = url.into();

    let mut request_builder = match method {
        Method::GET => client.get(&url),

        Method::POST => {
            if payload.is_none() {
                error!("Tried to POST without payload");
                return None;
            }

            client.post(&url).json(&payload)
        }

        _ => {
            error!("Unsupported HTTP method");
            return None;
        }
    };

    if let Some(headers) = headers {
        request_builder = request_builder.headers(headers);
    }

    let Ok(response) = request_builder.send() else {
        error!("Couldn't make a request to {url}");
        return None;
    };

    let Ok(text) = response.text() else {
        error!("Couldn't read text from response");
        return None;
    };

    // let status_code = response.status();
    // if  status_code != reqwest::StatusCode::OK {
    //     info!("[{status_code}] - {text}");
    //     return None;
    // };

    serde_json::from_str(&text).map_or(None, |r| r)
}
