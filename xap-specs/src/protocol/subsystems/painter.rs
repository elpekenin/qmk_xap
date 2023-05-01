use std::cmp::{max, min};

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::request::XAPRequest;

#[derive(BinWrite, Clone, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct HSVColor {
    pub hue: u8,
    pub sat: u8,
    pub val: u8,
}

impl HSVColor {
    #[must_use]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let c_max = f32::from(max(max(r, g), b)) / 255.0;
        let c_min = f32::from(min(min(r, g), b)) / 255.0;
        let delta = c_max - c_min;

        let r2 = f32::from(r) / 255.0;
        let g2 = f32::from(g) / 255.0;
        let b2 = f32::from(b) / 255.0;

        let h = {
            if delta == 0.0 {
                0
            } else if c_max == r2 {
                60 * ((((g2 - b2) / delta) as u8) % 6)
            } else if c_max == g2 {
                60 * ((((b2 - r2) / delta) as u8) + 2)
            } else {
                60 * ((((r2 - g2) / delta) as u8) + 4)
            }
        };
        let s = if c_max == 0.0 { 0.0 } else { delta / c_max };
        let v = c_max;

        // scale to 255, 255, 255 range
        let hue = (u16::from(h) * 255 / 360) as u8;
        let sat = (s * 255.0) as u8;
        let val = (v * 255.0) as u8;

        Self { hue, sat, val }
    }
}

// ==============================
// 0x3 0x2 0x1
#[derive(BinWrite, Debug)]
pub struct PainterDrawClear(pub u8);

impl XAPRequest for PainterDrawClear {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x1]
    }
}

// ==============================
// 0x3 0x2 0x2
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterPixel {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub color: HSVColor,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawPixel(pub PainterPixel);

impl XAPRequest for PainterDrawPixel {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x2]
    }
}

// ==============================
// 0x3 0x2 0x3
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterLine {
    pub screen_id: u8,
    pub x0: u16,
    pub y0: u16,
    pub x1: u16,
    pub y1: u16,
    pub color: HSVColor,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawLine(pub PainterLine);

impl XAPRequest for PainterDrawLine {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x3]
    }
}

// ==============================
// 0x3 0x2 0x4
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterRect {
    pub screen_id: u8,
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub color: HSVColor,
    pub filled: u8,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawRect(pub PainterRect);

impl XAPRequest for PainterDrawRect {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x4]
    }
}

// ==============================
// 0x3 0x2 0x5
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterCircle {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub radius: u16,
    pub color: HSVColor,
    pub filled: u8,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawCircle(pub PainterCircle);

impl XAPRequest for PainterDrawCircle {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x5]
    }
}

// ==============================
// 0x3 0x2 0x6
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterEllipse {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub sizex: u16,
    pub sizey: u16,
    pub color: HSVColor,
    pub filled: u8,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawEllipse(pub PainterEllipse);

impl XAPRequest for PainterDrawEllipse {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x6]
    }
}

// ==============================
// 0x3 0x2 0x7
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterImage {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub img: u8,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawImage(pub PainterImage);

impl XAPRequest for PainterDrawImage {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x7]
    }
}

// ==============================
// 0x3 0x2 0x8
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterImageRecolor {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub img: u8,
    pub fg_color: HSVColor,
    pub bg_color: HSVColor,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawImageRecolor(pub PainterImageRecolor);

impl XAPRequest for PainterDrawImageRecolor {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x8]
    }
}

// ==============================
// 0x3 0x2 0x9
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterAnimate {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub img: u8,
}
// so far animate data equals regular image data, but keeping a sep struct

#[derive(BinWrite, Debug)]
pub struct PainterDrawAnimate(pub PainterAnimate);

impl XAPRequest for PainterDrawAnimate {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x9]
    }
}

// ==============================
// 0x3 0x2 0xA
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterAnimateRecolor {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub img: u8,
    pub fg_color: HSVColor,
    pub bg_color: HSVColor,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawAnimateRecolor(pub PainterAnimateRecolor);

impl XAPRequest for PainterDrawAnimateRecolor {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0xA]
    }
}

// ==============================
// 0x3 0x2 0xB
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterText {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub font: u8,
    pub text: Vec<u8>,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawText(pub PainterText);

impl XAPRequest for PainterDrawText {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0xB]
    }
}

// ==============================
// 0x3 0x2 0xC
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterTextRecolor {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub font: u8,
    pub fg_color: HSVColor,
    pub bg_color: HSVColor,
    pub text: Vec<u8>,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawTextRecolor(pub PainterTextRecolor);

impl XAPRequest for PainterDrawTextRecolor {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0xC]
    }
}

// ==============================
// 0x3 0x2 0xD
#[derive(BinWrite, BinRead, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterGeometry {
    pub width: u16,
    pub height: u16,
    pub rotation: u8,
    pub offset_x: u16,
    pub offset_y: u16,
}

#[derive(BinWrite, Debug)]
pub struct PainterGetGeometry(pub u8);

impl XAPRequest for PainterGetGeometry {
    type Response = PainterGeometry;

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0xD]
    }
}

// ==============================
// 0x3 0x2 0xF
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterViewport {
    pub screen_id: u8,
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
}

#[derive(BinWrite, Debug)]
pub struct PainterSetViewport(pub PainterViewport);

impl XAPRequest for PainterSetViewport {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0xF]
    }
}

// ==============================
// 0x3 0x2 0x10
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterPixdata {
    pub screen_id: u8,
    pub pixels: Vec<u8>,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawPixdata(pub PainterPixdata);

impl XAPRequest for PainterDrawPixdata {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x10]
    }
}

// ==============================
// 0x3 0x2 0x11
#[derive(BinWrite, Debug)]
pub struct PainterSurfaceDrawText(pub PainterText);

impl XAPRequest for PainterSurfaceDrawText {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x11]
    }
}

// ==============================
// 0x3 0x2 0x12
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterTextWidth {
    pub font: u8,
    pub text: Vec<u8>,
}

#[derive(BinWrite, Debug)]
pub struct PainterGetTextWidth(pub PainterTextWidth);

impl XAPRequest for PainterGetTextWidth {
    type Response = i16;

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x12]
    }
}

// ==============================
// 0x3 0x2 0x13
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterScrollingText {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
    pub font: u8,
    pub n_chars: u8,
    pub delay: u16,
    pub text: Vec<u8>,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawScrollingText(pub PainterScrollingText);

impl XAPRequest for PainterDrawScrollingText {
    type Response = u8;

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x13]
    }
}

// ==============================
// 0x3 0x2 0x14

#[derive(BinWrite, Debug)]
pub struct PainterDrawStopScrollingText(pub u8);

impl XAPRequest for PainterDrawStopScrollingText {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x14]
    }
}

// ==============================
// 0x3 0x2 0x15

#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterExtendScrollingText {
    pub token: u8,
    pub text: Vec<u8>,
}

#[derive(BinWrite, Debug)]

pub struct PainterDrawExtendScrollingText(pub PainterExtendScrollingText);


impl XAPRequest for PainterDrawExtendScrollingText {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x3, 0x2, 0x15]
    }
}