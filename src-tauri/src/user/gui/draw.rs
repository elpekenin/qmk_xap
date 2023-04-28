use crate::xap::hid::XAPDevice;
use xap_specs::protocol::painter::*;

use super::{BG_COLOR, HSV_BLACK, HSV_WHITE};

pub fn image(device: &XAPDevice, screen_id: u8, x: u16, y: u16, img: u8) {
    let _ = device.query(PainterDrawImage(PainterImage {
        screen_id,
        x,
        y,
        img,
    }));
}

pub fn image_recolor(
    device: &XAPDevice,
    screen_id: u8,
    x: u16,
    y: u16,
    img: u8,
    fg_color: HSVColor,
    bg_color: HSVColor,
) {
    let _ = device.query(PainterDrawImageRecolor(PainterImageRecolor {
        screen_id,
        x,
        y,
        img,
        fg_color,
        bg_color,
    }));
}

fn normalize_string(input: impl Into<Vec<u8>>) -> Vec<u8> {
    let text = String::from_utf8(input.into())
        .unwrap()
        .replace(['á', 'ä', 'à'], "a")
        .replace(['Á', 'Ä', 'À'], "A")
        .replace(['é', 'ë', 'è'], "e")
        .replace(['É', 'Ë', 'È'], "E")
        .replace(['í', 'ï', 'ì'], "i")
        .replace(['Í', 'Ï', 'Ì'], "I")
        .replace(['ó', 'ö', 'ò'], "o")
        .replace(['Ó', 'Ö', 'Ò'], "O")
        .replace(['ú', 'ü', 'ù'], "u")
        .replace(['Ú', 'Ü', 'Ù'], "U")
        .replace('ñ', "n")
        .replace('Ñ', "N")
        .replace('ç', "c")
        .replace('Ç', "C")
        .replace(['&', '¡', '¿'], "");

    let mut array = text.as_bytes();

    if array.len() > 40 {
        array = &array[..40];
    }

    array.to_vec()
}

pub fn text_recolor(
    device: &XAPDevice,
    screen_id: u8,
    x: u16,
    y: u16,
    font: u8,
    fg_color: HSVColor,
    bg_color: HSVColor,
    text: impl Into<Vec<u8>>,
) {
    let text = normalize_string(text);

    let _ = device.query(PainterDrawTextRecolor(PainterTextRecolor {
        screen_id,
        x,
        y,
        font,
        fg_color,
        bg_color,
        text,
    }));
}

pub fn rect(
    device: &XAPDevice,
    screen_id: u8,
    left: u16,
    top: u16,
    right: u16,
    bottom: u16,
    color: HSVColor,
    filled: impl Into<u8>,
) {
    let filled = filled.into();
    let _ = device.query(PainterDrawRect(PainterRect {
        screen_id,
        left,
        top,
        right,
        bottom,
        color,
        filled,
    }));
}

pub fn geometry(device: &XAPDevice, screen_id: u8) -> PainterGeometry {
    device.query(PainterGetGeometry(screen_id)).unwrap()
}

pub fn viewport(device: &XAPDevice, screen_id: u8, left: u16, top: u16, right: u16, bottom: u16) {
    let _ = device.query(PainterSetViewport(PainterViewport {
        screen_id,
        left,
        top,
        right,
        bottom,
    }));
}

pub fn pixdata(device: &XAPDevice, screen_id: u8, pixels: impl Into<Vec<u8>>) {
    let pixels = pixels.into();
    let _ = device.query(PainterDrawPixdata(PainterPixdata { screen_id, pixels }));
}

pub fn text_width(device: &XAPDevice, font: u8, text: impl Into<Vec<u8>>) -> u16 {
    let text = normalize_string(text);
    let value = device
        .query(PainterGetTextWidth(PainterTextWidth { font, text }))
        .unwrap();

    if value < 0 {
        u16::MAX
    } else {
        value as u16
    }
}

pub fn clear(device: &XAPDevice, screen_id: u8) {
    let geometry = geometry(device, screen_id);
    rect(
        device,
        screen_id,
        0,
        0,
        geometry.width,
        geometry.height,
        HSV_BLACK,
        true,
    );
}

pub fn scrolling_text(
    device: &XAPDevice,
    screen_id: u8,
    x: u16,
    y: u16,
    font: u8,
    text: impl Into<Vec<u8>>,
    n_chars: u8,
    delay: u16,
) -> u8 {
    let text = normalize_string(text);

    // force this to draw onto the real display (__ili9163)
    //    * otherwise we run into issues due to viewport
    //    * improves performance as we avoid having to flush the scrolling text changes
    //        (which happen often) from surface to real display
    let screen_id = 2;

    device
        .query(PainterDrawScrollingText(PainterScrollingText {
            screen_id,
            x,
            y,
            font,
            n_chars,
            delay,
            text,
        }))
        .unwrap()
}

pub fn centered_text(
    device: &XAPDevice,
    screen_id: u8,
    x: u16,
    y: u16,
    font: u8,
    text: impl Into<Vec<u8>>,
) {
    let text = normalize_string(text);

    let geometry = geometry(device, screen_id);
    let textwidth = text_width(device, font, text.clone());

    // guard clause, doesn't fit
    if x + textwidth / 2 > geometry.width || textwidth / 2 > x {
        return;
    }

    let x = x - textwidth / 2;
    let fg_color = HSV_WHITE;
    let bg_color = HSV_BLACK;

    text_recolor(device, screen_id, x, y, font, fg_color, bg_color, text);
}

pub fn centered_or_scrolling_text(
    device: &XAPDevice,
    screen_id: u8,
    y: u16,
    font: u8,
    text: impl Into<Vec<u8>>,
) -> Option<u8> {
    let text = normalize_string(text);

    let geometry = geometry(device, screen_id);
    let textwidth = text_width(device, font, text.clone());

    if textwidth > geometry.width {
        let x = 0;
        let n_chars = 18; // hardcoded based on my screen
        let delay = 300;
        return Some(scrolling_text(
            device, screen_id, x, y, font, text, n_chars, delay,
        ));
    }

    centered_text(device, screen_id, geometry.width/2, y, font, text);

    return None;
}

pub fn stop_scrolling_text(device: &XAPDevice, token: Option<u8>) {
    match token {
        Some(token) => {
            device.query(PainterDrawStopScrollingText(token));
        }
        None => {}
    };
}
