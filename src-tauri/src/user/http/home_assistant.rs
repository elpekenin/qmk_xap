use crate::user::http;
use reqwest::{
    header::{self, HeaderMap},
    Method,
};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn get_state(entity_id: impl Into<String>) -> Map<String, Value> {
    let entity_id = entity_id.into();

    let hasst_token = std::env::var("HASST_TOKEN").unwrap();
    let hasst_base_url =
        std::env::var("HASST_BASE_URL").unwrap_or_else(|_| "http://localhost:8123".to_string());
    let url = format!("{hasst_base_url}/api/states/{entity_id}");

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {hasst_token}").parse().unwrap(),
    );

    http::request(Method::GET, url, Some(headers), None).unwrap()
}

pub fn set_light_intensity(level: u16) {
    let level = 255 / 5 * level;

    let hasst_token = std::env::var("HASST_TOKEN").unwrap();
    let hasst_base_url =
        std::env::var("HASST_BASE_URL").unwrap_or("http://localhost:8123".to_string());
    let lightbulb = std::env::var("LIGHTBULB_ID").unwrap();

    let url = format!("{hasst_base_url}/api/services/light/turn_on");

    let mut payload = HashMap::new();
    payload.insert("entity_id", lightbulb);
    payload.insert("brightness", level.to_string());

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {hasst_token}").parse().unwrap(),
    );

    http::request(Method::POST, url, Some(headers), Some(payload));
}
