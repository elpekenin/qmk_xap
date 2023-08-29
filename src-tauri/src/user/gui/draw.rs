use crate::{
    user::gui::{HSV_BLACK, HSV_WHITE},
    xap::hid::XAPDevice,
};
use xap_specs::protocol::{
    keymap::{KeyCoords, KeyPosition},
    painter::*,
};

use log::error;

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
    String::from_utf8(input.into())
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
        .replace(['&', '¡', '¿'], "")
        .as_bytes()
        .to_vec()
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
    let text = normalize_string(text)
        .into_iter()
        .take(PainterTextRecolor::text_size())
        .collect();

    let _ = device.query(PainterDrawTextRecolor(PainterTextRecolor {
        screen_id,
        x,
        y,
        font,
        fg_color,
        bg_color,
        text,
        ..Default::default()
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

pub fn textwidth(device: &XAPDevice, font: u8, text: &Vec<u8>) -> u16 {
    let text = normalize_string(text.to_owned())
        .into_iter()
        .take(PainterTextWidth::text_size())
        .collect();

    match device.query(PainterGetTextWidth(PainterTextWidth {
        font,
        text,
        ..Default::default()
    })) {
        Ok(value) => {
            if value < 0 {
                u16::MAX
            } else {
                value as u16
            }
        }
        Err(_) => u16::MAX,
    }
}

pub fn clear(device: &XAPDevice, screen_id: u8) {
    let geometry = geometry(device, screen_id);

    rect(
        device,
        screen_id,
        0,
        0,
        geometry.width - 1,
        geometry.height - 1,
        HSV_BLACK,
        true,
    );
}

fn extend_text(device: &XAPDevice, token: u8, text: impl Into<Vec<u8>>) {
    let text = text.into();

    device
        .query(PainterDrawExtendScrollingText(PainterExtendScrollingText {
            token,
            text,
            ..Default::default()
        }))
        .unwrap();
}

pub fn scrolling_text(
    device: &XAPDevice,
    screen_id: u8,
    x: u16,
    y: u16,
    font: u8,
    text: impl Into<Vec<u8>>,
    delay: u16,
) -> Option<u8> {
    let text = normalize_string(text);
    let msg_size = PainterScrollingText::text_size();

    let first: Vec<u8> = text.clone().into_iter().take(msg_size).collect();

    let n_chars = match (screen_id, font) {
        (0, 1) => 18,
        (0, 0) => 7,
        (1, 0) => 18,
        (s, f) => {
            error!("Combination not configured. Screen {s}, font: {f}");
            return None;
        }
    };

    let token = device
        .query(PainterDrawScrollingText(PainterScrollingText {
            screen_id,
            x,
            y,
            font,
            n_chars,
            delay,
            text: first,
            ..Default::default()
        }))
        .unwrap();

    if text.len() > msg_size {
        text[msg_size..]
            .chunks(PainterExtendScrollingText::text_size())
            .for_each(|text| extend_text(device, token, text));
    }

    Some(token)
}

pub fn centered_text(
    device: &XAPDevice,
    screen_id: u8,
    x: u16,
    y: u16,
    font: u8,
    text: impl Into<Vec<u8>>,
) {
    let text = text.into();
    let geometry = geometry(device, screen_id);
    let textwidth = textwidth(device, font, &text);

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
    let text = text.into();
    let geometry = geometry(device, screen_id);
    let textwidth = textwidth(device, font, &text);

    if textwidth > geometry.width {
        let x = 0;
        let delay = 300;
        return scrolling_text(device, screen_id, x, y, font, text, delay);
    }

    centered_text(device, screen_id, geometry.width / 2, y, font, text);

    None
}

pub fn stop_scrolling_text(device: &XAPDevice, token: Option<u8>) {
    if let Some(token) = token {
        let _ = device.query(PainterDrawStopScrollingText(token));
    };
}

pub fn draw_layer(device: &XAPDevice, layer: u8) {
    for row in &device.key_info()[layer as usize] {
        for key in row {
            match key {
                None => continue,
                Some(info) => {
                    // physichal position
                    let KeyCoords { x, y, w: _, h: _ } = info.coords;
                    let size = 23;
                    let x = x as u16 * size;
                    let y = y as u16 * size;

                    // electrical position
                    let KeyPosition { layer, row, col } = info.position;

                    let _ = device.query(PainterDrawKeycode(PainterKeycode {
                        screen_id: 1,
                        x,
                        y,
                        font: 1,
                        layer,
                        row,
                        col,
                    }));
                }
            }
        }
    }
}
