use std::sync::Arc;

use parking_lot::Mutex;
use tauri::State;
use uuid::Uuid;

use xap_specs::{
    protocol::painter::{
        PainterDrawCircle, PainterDrawEllipse, PainterDrawLine, PainterDrawPixel, PainterDrawRect, PainterDrawText,
        PainterCircle, PainterEllipse, PainterLine, PainterPixel, PainterRect, PainterText, 
    }
};

use crate::xap::{hid::XAPClient, ClientResult};

#[tauri::command]
pub(crate) async fn painter_pixel(
    id: Uuid,
    arg: PainterPixel,
    state: State<'_, Arc<Mutex<XAPClient>>>,
) -> ClientResult<()> {
    state.lock().query(id,PainterDrawPixel(arg))
}

#[tauri::command]
pub(crate) async fn painter_line(
    id: Uuid,
    arg: PainterLine,
    state: State<'_, Arc<Mutex<XAPClient>>>,
) -> ClientResult<()> {
    state.lock().query(id,PainterDrawLine(arg))
}

#[tauri::command]
pub(crate) async fn painter_rect(
    id: Uuid,
    arg: PainterRect,
    state: State<'_, Arc<Mutex<XAPClient>>>,
) -> ClientResult<()> {
    state.lock().query(id,PainterDrawRect(arg))
}

#[tauri::command]
pub(crate) async fn painter_circle(
    id: Uuid,
    arg: PainterCircle,
    state: State<'_, Arc<Mutex<XAPClient>>>,
) -> ClientResult<()> {
    state.lock().query(id,PainterDrawCircle(arg))
}

#[tauri::command]
pub(crate) async fn painter_ellipse(
    id: Uuid,
    arg: PainterEllipse,
    state: State<'_, Arc<Mutex<XAPClient>>>,
) -> ClientResult<()> {
    state.lock().query(id,PainterDrawEllipse(arg))
}

#[tauri::command]
pub(crate) async fn painter_text(
    id: Uuid,
    arg: PainterText,
    state: State<'_, Arc<Mutex<XAPClient>>>,
) -> ClientResult<()> {
    state.lock().query(id,PainterDrawText(arg))
}
