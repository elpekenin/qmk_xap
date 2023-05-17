use crate::user::http::FRAGMENT;
use percent_encoding::utf8_percent_encode;
use reqwest::Method;

pub fn text(text: impl Into<String>) -> Result<(), std::env::VarError> {
    let text = text.into();

    let tg_token = std::env::var("TG_TOKEN")?;
    let tg_id = std::env::var("TG_ID")?;

    let escaped_text = utf8_percent_encode(&text, FRAGMENT).to_string();
    let url = format!(
        "https://api.telegram.org/bot{tg_token}/sendMessage?chat_id={tg_id}&text={escaped_text}"
    );

    super::request(Method::GET, url, None, None);

    Ok(())
}
