use binrw::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::request::XAPRequest;

// ==============================
// TODO: Missing a couple routes
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct HSVColor {
    pub hue: u8,
    pub sat: u8,
    pub val: u8,
}

// ==============================
// 0x2 0x2 0x1
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
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
pub struct PainterDrawClear(pub PainterGeometry);

impl XAPRequest for PainterDrawClear {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x2, 0x2, 0x1]
    }
}

// ==============================
// 0x2 0x2 0x2
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterPixel {
    pub dev: u8,
    pub x: u16,
    pub y: u16,
    pub color: HSVColor,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawPixel(pub PainterPixel);

impl XAPRequest for PainterDrawPixel {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x2, 0x2, 0x2]
    }
}

// ==============================
// 0x2 0x2 0x3
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterLine {
    pub dev: u8,
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
        &[0x2, 0x2, 0x3]
    }
}

// ==============================
// 0x2 0x2 0x4
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterRect {
    pub dev: u8,
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
        &[0x2, 0x2, 0x4]
    }
}

// ==============================
// 0x2 0x2 0x5
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterCircle {
    pub dev: u8,
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
        &[0x2, 0x2, 0x5]
    }
}

// ==============================
// 0x2 0x2 0x6
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterEllipse {
    pub dev: u8,
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
        &[0x2, 0x2, 0x6]
    }
}

// ==============================
// 0x2 0x2 0x7
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterImage {
    pub dev: u8,
    pub x: u16,
    pub y: u16,
    pub img: u8,
}

#[derive(BinWrite, Debug)]
pub struct PainterDrawImage(pub PainterImage);

impl XAPRequest for PainterDrawImage {
    type Response = ();

    fn id() -> &'static [u8] {
        &[0x2, 0x2, 0x7]
    }
}

// ==============================
// 0x2 0x2 0x8
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterImageRecolor {
    pub dev: u8,
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
        &[0x2, 0x2, 0x8]
    }
}

// ==============================
// 0x2 0x2 0x9
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterAnimate { 
    pub dev: u8,
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
        &[0x2, 0x2, 0x9]
    }
}

// ==============================
// 0x2 0x2 0xA
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterAnimateRecolor {
    pub dev: u8,
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
        &[0x2, 0x2, 0xA]
    }
}

// ==============================
// 0x2 0x2 0xB
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterText {
    pub dev: u8,
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
        &[0x2, 0x2, 0xB]
    }
}

// ==============================
// 0x2 0x2 0xC
#[derive(BinWrite, Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../bindings/")]
pub struct PainterTextRecolor {
    pub dev: u8,
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
        &[0x2, 0x2, 0xC]
    }
}