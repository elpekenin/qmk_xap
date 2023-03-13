use crate::xap::hid::XAPDevice;
use log::info;
use xap_specs::{protocol::painter::*, request::XAPRequest};

use super::{HSV_BLACK, HSV_WHITE};

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
    let text = String::from_utf8(input.into()).unwrap().to_string()
        .replace('á', "a")
        .replace('ä', "a")
        .replace('à', "a")
        .replace('Á', "A")
        .replace('Ä', "A")
        .replace('À', "A")
        .replace('é', "e")
        .replace('ë', "e")
        .replace('è', "e")
        .replace('É', "E")
        .replace('Ë', "E")
        .replace('À', "E")
        .replace('í', "i")
        .replace('ï', "i")
        .replace('ì', "i")
        .replace('Í', "I")
        .replace('Ï', "I")
        .replace('Ì', "I")
        .replace('ó', "o")
        .replace('ö', "o")
        .replace('ò', "o")
        .replace('Ó', "O")
        .replace('Ö', "O")
        .replace('Ò', "O")
        .replace('ú', "u")
        .replace('ü', "u")
        .replace('ù', "u")
        .replace('Ú', "U")
        .replace('Ü', "U")
        .replace('Ù', "U")
        .replace('ñ', "n")
        .replace('Ñ', "N")
        .replace('ç', "c")
        .replace('Ç', "C")
        .replace("&", "");

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

pub fn text(device: &XAPDevice, screen_id: u8, x: u16, y: u16, font: u8, text: impl Into<Vec<u8>>) {
    text_recolor(device, screen_id, x, y, font, HSV_BLACK, HSV_WHITE, text);
}

pub fn surface_text(
    device: &XAPDevice,
    screen_id: u8,
    x: u16,
    y: u16,
    font: u8,
    text: impl Into<Vec<u8>>,
) {
    let text = normalize_string(text);

    let _ = device.query(PainterSurfaceDrawText(PainterText {
        screen_id,
        x,
        y,
        font,
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

pub fn pixel(device: &XAPDevice, screen_id: u8, x: u16, y: u16, color: HSVColor) {
    let _ = device.query(PainterDrawPixel(PainterPixel {
        screen_id,
        x,
        y,
        color,
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
    let value = device.query(PainterGetTextWidth(PainterTextWidth { font, text })).unwrap();

    if value < 0 { u16::MAX } else { value as u16 }
}
