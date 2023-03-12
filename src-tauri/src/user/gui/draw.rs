use crate::xap::hid::XAPDevice;
use log::info;
use xap_specs::{
    protocol::painter::*,
    request::XAPRequest,
};

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

fn normalize_string(input: String) -> String {
    let mut text = input.clone();

    text = text.replace('á', "a");
    text = text.replace('ä', "a");
    text = text.replace('à', "a");
    text = text.replace('Á', "A");
    text = text.replace('Ä', "A");
    text = text.replace('À', "A");

    text = text.replace('é', "e");
    text = text.replace('ë', "e");
    text = text.replace('è', "e");
    text = text.replace('É', "E");
    text = text.replace('Ë', "E");
    text = text.replace('À', "E");

    text = text.replace('í', "i");
    text = text.replace('ï', "i");
    text = text.replace('ì', "i");
    text = text.replace('Í', "I");
    text = text.replace('Ï', "I");
    text = text.replace('Ì', "I");

    text = text.replace('ó', "o");
    text = text.replace('ö', "o");
    text = text.replace('ò', "o");
    text = text.replace('Ó', "O");
    text = text.replace('Ö', "O");
    text = text.replace('Ò', "O");

    text = text.replace('ú', "u");
    text = text.replace('ü', "u");
    text = text.replace('ù', "u");
    text = text.replace('Ú', "U");
    text = text.replace('Ü', "U");
    text = text.replace('Ù', "U");

    text = text.replace('ñ', "n");
    text = text.replace('N', "N");

    text = text.replace('ç', "c");
    text = text.replace('Ç', "C");

    text.to_owned()
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
    let input = String::from_utf8(text.into()).unwrap();
    let normalized = normalize_string(input);
    let text = normalized.as_bytes().to_vec();

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

pub fn surface_text(
    device: &XAPDevice,
    screen_id: u8,
    x: u16,
    y: u16,
    font: u8,
    text: impl Into<Vec<u8>>,
) {
    let input = String::from_utf8(text.into()).unwrap();
    let normalized = normalize_string(input);
    let text = normalized.as_bytes().to_vec();

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
