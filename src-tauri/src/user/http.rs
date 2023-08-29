pub mod github;

#[allow(unused)]
pub mod home_assistant;

#[allow(unused)]
pub mod telegram;

pub mod weather;

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

pub type ResponseT = Option<Vec<Map<String, Value>>>;

fn request(
    method: Method,
    url: impl Into<String>,
    headers: Option<HeaderMap>,
    payload: Option<HashMap<&str, String>>,
) ->  ResponseT {
    let client = reqwest::blocking::Client::new();

    let url = url.into();

    let mut request_builder = match method {
        Method::GET => client.get(&url),

        Method::POST => {
            if payload.is_none() {
                log::error!("Tried to POST without payload");
                return None;
            }

            client.post(&url).json(&payload)
        }

        _ => {
            log::error!("Unsupported HTTP method");
            return None;
        }
    };

    if let Some(headers) = headers {
        request_builder = request_builder.headers(headers);
    }

    let Ok(response) = request_builder.send() else {
        log::error!("Couldn't make a request to {url}");
        return None;
    };

    let Ok(mut text) = response.text() else {
        log::error!("Couldn't read text from response");
        return None;
    };

    // convert plain mapping to 1-element array
    if !text.starts_with('[') {
        text = format!("[{text}]");
    }

    serde_json::from_str(&text).map_or(None, |r| r)
}
