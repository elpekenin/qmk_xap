use crate::user;

use reqwest::{
    header::{self, HeaderMap},
    Method,
};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn get_state(entity_id: impl Into<String>) -> super::ResponseT {
    let entity_id = entity_id.into();

    let hasst_token = user::get_var("HASST_TOKEN");
    let hasst_base_url = user::get_var("HASST_BASE_URL");
    let url = format!("{hasst_base_url}/api/states/{entity_id}");

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {hasst_token}").parse().unwrap(),
    );

    super::request(Method::GET, url, Some(headers), None)
}

pub fn set_light_intensity(level: u16) {
    let level = 255 / 5 * level;

    let hasst_token = user::get_var("HASST_TOKEN");
    let hasst_base_url = user::get_var("HASST_BASE_URL");
    let lightbulb = user::get_var("LIGHTBULB_ID");

    let url = format!("{hasst_base_url}/api/services/light/turn_on");

    let mut payload = HashMap::new();
    payload.insert("entity_id", lightbulb);
    payload.insert("brightness", level.to_string());

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {hasst_token}").parse().unwrap(),
    );

    super::request(Method::POST, url, Some(headers), Some(payload));
}
