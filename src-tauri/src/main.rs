#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
mod commands;
mod aggregation;
mod events;
mod user;
mod xap;

use std::collections::HashMap;
use std::hash::Hash;
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
use user::UserData;
use uuid::Uuid;
use xap::hid::XAPClient;
use xap::ClientResult;
use xap_specs::constants::XAPConstants;
use xap_specs::protocol::UserBroadcast;

fn shutdown_event_loop<R: Runtime>(sender: Sender<XAPEvent>) -> TauriPlugin<R> {
    Builder::new("event loop shutdown")
        .on_event(move |_, event| {
            if let RunEvent::ExitRequested { .. } = event {
                sender.send(XAPEvent::Exit).unwrap();
            }
        })
        .build()
}

// slightly oversized
const RECEPTION_BUFFER_INITIAL_SIZE: usize = 100;

#[derive(Default)]
struct LogBuffers {
    names: HashMap<Uuid, String>,
    buffers: HashMap<Uuid, String>,
}

impl LogBuffers {
    fn get_name(&mut self, state: &XAPClient, id: &Uuid) -> String {
        match self.names.get(id) {
            Some(value) => value.to_owned(),
            None => {
                let xap_info = state.get_device(&id).unwrap().xap_info();

                let name = format!("{}:{}", xap_info.qmk.manufacturer, xap_info.qmk.product_name);

                self.names.insert(id.to_owned(), name.clone());
                name
            }
        }
    }

    fn get_buffer(&mut self, id: &Uuid) -> &mut String {
        self.buffers.entry(*id).or_insert(String::with_capacity(RECEPTION_BUFFER_INITIAL_SIZE))
    }

    fn reset_buffer(&mut self, id: &Uuid) {
        let _ = self.buffers.insert(*id, String::with_capacity(RECEPTION_BUFFER_INITIAL_SIZE));
    }

    fn append_text(&mut self, id: &Uuid, text: impl Into<String>) {
        let text = text.into();

        let buffer = self.get_buffer(id);
        buffer.push_str(&text);
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn log(&mut self, state: &XAPClient, id: &Uuid, log: impl Into<String>) {
        let log = log.into();

        let name = self.get_name(state, id);

        if !log.contains('\n') {
            self.append_text(id, log);
            return;
        }

        let parts: Vec<&str> = log.split('\n').collect();
        let n_parts = parts.len();

        for i in 0..n_parts {
            let part = parts[i];
            self.append_text(id, part);

            if i != (n_parts-1) {
                println!("[{name}] {}", self.get_buffer(id));
                self.reset_buffer(id);
            }
        }
    }

}


fn start_event_loop(
    app: AppHandle,
    state: Arc<Mutex<XAPClient>>,
    event_channel: Receiver<XAPEvent>,
    user_data: Arc<Mutex<UserData>>,
) {
    let _ = std::thread::spawn(move || {
        let ticker = tick(Duration::from_millis(500));
        let state = state;

        let mut log_buffers = LogBuffers::new();

        info!("started event loop");
        'event_loop: loop {
            select! {
                recv(event_channel) -> msg => {
                    match msg {
                        Ok(XAPEvent::Exit) => {
                            info!("received shutdown signal, exiting!");
                            let client = state.lock();
                            let mut user_data = user_data.lock();
                            user::on_close(&client, &mut user_data);
                            break 'event_loop;
                        },
                        Ok(XAPEvent::LogReceived{id, log}) => {
                            let client = state.lock();

                            log_buffers.log(&client, &id, &log);

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

                                let mut user_data = user_data.lock();
                                user::new_device(device, &mut user_data);

                                // prevents drawing before the screen gets cleared
                                // otherwise we'll remove some of our own drawing
                                user_data.connected = true;
                            }
                        },
                        Ok(XAPEvent::RemovedDevice(id)) => {
                            info!("removed device - notifying frontend!");
                            app.emit_all("removed-device", FrontendEvent::RemovedDevice{ id }).unwrap();

                            let mut user_data = user_data.lock();
                            user::removed_device(&id, &mut user_data);
                        },
                        Ok(XAPEvent::AnnounceAllDevices) => {
                            let mut state = state.lock();
                            info!("announcing all xap devices to the frontend");
                            if matches!(state.enumerate_xap_devices(), Ok(())) {
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
                        Ok(XAPEvent::ReceivedUserBroadcast{broadcast, id}) => {
                            // Parse raw data
                            let user_broadcast: UserBroadcast = if let Ok(broadcast) = broadcast.into_xap_broadcast() {
                                broadcast
                            } else {
                                log::error!("Couldn't parse raw broadcast into user broadcast");
                                return;
                            };

                            let state = state.lock();
                            let device = state.get_device(&id).unwrap();
                            let mut user_data = user_data.lock();
                            user::broadcast_callback(user_broadcast, device, &mut user_data);
                        },
                        Err(err) => {
                            error!("error receiving event {err}");
                        },
                    }

                },
                recv(ticker) -> msg => {
                    match msg {
                        Ok(_) => {
                            let mut state = state.lock();

                            if let Err(err) = state.enumerate_xap_devices() {
                                error!("failed to enumerate XAP devices: {err}");
                                return;
                            }

                            let mut user_data = user_data.lock();
                            user::housekeeping(&state, &mut user_data);
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
    user::pre_init();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let (event_channel_tx, event_channel_rx): (Sender<XAPEvent>, Receiver<XAPEvent>) = unbounded();

    let state = Arc::new(Mutex::new(XAPClient::new(event_channel_tx.clone())?));
    let cloned_state = state.clone();

    let user_data = Arc::new(Mutex::new(UserData::new()));
    let cloned_user_data = user_data.clone();

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
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        // Add system tray
        .system_tray(system_tray)
        // And its logic
        .on_system_tray_event(move |app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "hide" => {
                        app.get_window("main").unwrap().hide().unwrap();
                    }
                    "quit" => {
                        let mut user_data = cloned_user_data.lock();
                        user::on_close(&cloned_state.lock(), &mut user_data);
                        std::process::exit(0);
                    }
                    "show" => {
                        app.get_window("main").unwrap().show().unwrap();
                    }
                    _ => {}
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            xap_constants_get,
            secure_lock,
            secure_unlock,
            secure_status_get,
            jump_to_bootloader,
            reset_eeprom,
            painter_clear,
            painter_circle,
            painter_ellipse,
            painter_line,
            painter_pixel,
            painter_rect,
            painter_text,
            painter_geometry,
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
            let resource_path = app
                .path_resolver()
                .resolve_resource("../xap-specs/specs/constants/keycodes")
                .expect("failed to resolve resource");
            state
                .lock()
                .set_xap_constants(XAPConstants::new(resource_path)?);
            app.manage(state.clone());
            app.listen_global("frontend-loaded", move |_| {
                event_channel_tx_listen_frontend
                    .send(XAPEvent::AnnounceAllDevices)
                    .unwrap_or_else(|e| println!("{e}"));
            });
            start_event_loop(app.handle(), state, event_channel_rx, user_data);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
