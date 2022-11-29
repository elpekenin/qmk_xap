#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
mod commands;
mod aggregation;
mod events;
mod xap;
mod user;

use std::sync::Arc;
use std::time::Duration;

use crossbeam_channel::tick;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use env_logger::Env;
use log::{error, info};
use parking_lot::Mutex;

use tauri::{
    plugin::{Builder, TauriPlugin},
    CustomMenuItem, RunEvent, Runtime, SystemTray, SystemTrayMenu, SystemTrayMenuItem,
};
use tauri::{AppHandle, Manager, SystemTrayEvent};

use commands::*;
use events::{FrontendEvent, XAPEvent};
use xap::hid::XAPClient;
use xap::ClientResult;
use xap_specs::constants::XAPConstants;

fn shutdown_event_loop<R: Runtime>(sender: Sender<XAPEvent>) -> TauriPlugin<R> {
    Builder::new("event loop shutdown")
        .on_event(move |_, event| {
            if let RunEvent::ExitRequested { .. } = event {
                sender.send(XAPEvent::Exit).unwrap();
            }
        })
        .build()
}

fn start_event_loop(
    app: AppHandle,
    state: Arc<Mutex<XAPClient>>,
    event_channel: Receiver<XAPEvent>,
) {
    let _ = std::thread::spawn(move || {
        let ticker = tick(Duration::from_millis(500));
        let state = state;
        info!("started event loop");
        'event_loop: loop {
            select! {
                recv(event_channel) -> msg => {
                    match msg {
                        Ok(XAPEvent::Exit) => {
                            info!("received shutdown signal, exiting!");
                            break 'event_loop;
                        },
                        Ok(XAPEvent::LogReceived{id, log}) => {
                            info!("LOG: {id} {log}");
                                app.emit_all("log", FrontendEvent::LogReceived{ id, log }).unwrap();
                        },
                        Ok(XAPEvent::SecureStatusChanged{id, secure_status}) => {
                            info!("Secure status changed: {id} - {secure_status}");
                            app.emit_all("secure-status-changed", FrontendEvent::SecureStatusChanged{ id, secure_status }).unwrap();
                        },
                        Ok(XAPEvent::NewDevice(id)) => {
                            if let Ok(device) = state.lock().get_device(&id){
                                info!("detected new device - notifying frontend!");

                                app.emit_all("new-device", FrontendEvent::NewDevice{ device: device.as_dto() }).unwrap();
                                user::on_device_connection(device);
                            }
                        },
                        Ok(XAPEvent::RemovedDevice(id)) => {
                            info!("removed device - notifying frontend!");
                            app.emit_all("removed-device", FrontendEvent::RemovedDevice{ id }).unwrap();
                        },
                        Ok(XAPEvent::AnnounceAllDevices) => {
                            let mut state = state.lock();
                            info!("announcing all xap devices to the frontend");
                            if let Ok(()) = state.enumerate_xap_devices() {
                                for device in state.get_devices() {
                                    app.emit_all("new-device", FrontendEvent::NewDevice{ device: device.as_dto() }).unwrap();
                                }
                            }
                        },
                        Ok(XAPEvent::RxError) => {
                            if let Err(err) = state.lock().enumerate_xap_devices() {
                                error!("failed to enumerate XAP devices: {err}");
                            }
                        },
                        Ok(XAPEvent::HandleUserBroadcast{broadcast, id}) => {
                            user::broadcast_callback(broadcast, id, &state);
                        }
                        Err(err) => {
                            error!("error receiving event {err}");
                        },
                    }

                },
                recv(ticker) -> msg => {
                    match msg {
                        Ok(_) => {
                            if let Err(err) = state.lock().enumerate_xap_devices() {
                                error!("failed to enumerate XAP devices: {err}");
                            }
                        },
                        Err(err) => {
                            error!("failed receiving tick {err}");
                        }
                    }
                }
            }
        }
    });
}

fn main() -> ClientResult<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    user::on_init();

    let (event_channel_tx, event_channel_rx): (Sender<XAPEvent>, Receiver<XAPEvent>) = unbounded();
    let state = Arc::new(Mutex::new(XAPClient::new(
        event_channel_tx.clone(),
        XAPConstants::new("../xap-specs/specs/constants/keycodes".into())?,
    )?));
    let event_channel_tx_listen_frontend = event_channel_tx.clone();

    // System tray
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show"))
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(shutdown_event_loop(event_channel_tx))
        // Prevent window from closing
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close()
            },

            _ => {}
        })
        // Add system tray
        .system_tray(system_tray)
        // And its logic
        .on_system_tray_event(|app, event|
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "hide" => app.get_window("main").unwrap().hide().unwrap(),
                        "quit" => {
                            std::process::exit(0);
                        },
                        "show" => app.get_window("main").unwrap().show().unwrap(),
                        _ => {}
                    }
                },

                _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            xap_constants_get,
            secure_lock,
            secure_unlock,
            secure_status_get,
            jump_to_bootloader,
            reset_eeprom,
            painter_circle,
            painter_ellipse,
            painter_line,
            painter_pixel,
            painter_rect,
            painter_text,
            keycode_get,
            keycode_set,
            keymap_get,
            encoder_keycode_get,
            encoder_keycode_set,
            backlight_config_get,
            backlight_config_set,
            backlight_config_save,
            rgblight_config_get,
            rgblight_config_set,
            rgblight_config_save,
            rgbmatrix_config_get,
            rgbmatrix_config_set,
            rgbmatrix_config_save,
        ])
        .setup(move |app| {
            app.manage(state.clone());
            app.listen_global("frontend-loaded", move |_| {
                event_channel_tx_listen_frontend
                    .send(XAPEvent::AnnounceAllDevices)
                    .unwrap();
            });
            start_event_loop(app.handle(), state, event_channel_rx);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        // user::on_close();
    Ok(())
}
