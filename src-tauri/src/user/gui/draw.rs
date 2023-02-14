use crate::xap::hid::XAPDevice;
use xap_specs::protocol::painter::{
    HSVColor, PainterDrawImage, PainterDrawImageRecolor, PainterDrawPixel, PainterDrawRect,
    PainterDrawTextRecolor, PainterImage, PainterImageRecolor, PainterPixel, PainterRect,
    PainterTextRecolor,
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
    let text = text.into();
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

pub fn pixel(device: &XAPDevice, screen_id: u8, x: u16, y: u16, color: HSVColor) {
    let _ = device.query(PainterDrawPixel(PainterPixel {
        screen_id,
        x,
        y,
        color,
    }));
}
