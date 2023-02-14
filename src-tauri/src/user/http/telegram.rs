use crate::user::http::FRAGMENT;
use percent_encoding::utf8_percent_encode;
use reqwest::Method;

pub fn text(text: impl Into<String>) {
    let text = text.into();

    let tg_token = std::env::var("TG_TOKEN").unwrap();
    let tg_id = std::env::var("TG_ID").unwrap();

    let escaped_text = utf8_percent_encode(&text, FRAGMENT).to_string();
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        tg_token, tg_id, escaped_text
    );

    super::request(Method::GET, url, None, None);
}
