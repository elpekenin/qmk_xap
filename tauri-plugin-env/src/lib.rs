use tauri::{
    AppHandle,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[tauri::command]
async fn get<R: Runtime>(
    key: String,
    _app: AppHandle<R>,
) -> Result<String, String> {
    let ret = std::env::var(key).map_err(|e| e.to_string())?;

    Ok(ret)
}

#[derive(Default)]
struct EnvState;

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    if cfg!(feature = "dotenv") {
        let _ = dotenvy::dotenv();
    }

    Builder::new("env")
        .invoke_handler(tauri::generate_handler![get])
        .setup(|app, _| {
            app.manage(EnvState::default());
            Ok(())
        })
        .build()
}
