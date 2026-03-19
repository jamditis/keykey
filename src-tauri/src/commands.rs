use crate::config::{schema::AppConfig, store};
use crate::monitor::tracker::{self, MonitorInfo};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

pub struct ConfigState(pub Mutex<AppConfig>);

#[tauri::command]
pub fn get_config(state: State<ConfigState>) -> AppConfig {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
pub fn save_config(
    config: AppConfig,
    state: State<ConfigState>,
    app: AppHandle,
) -> Result<(), String> {
    store::save_config(&config)?;
    *state.0.lock().unwrap() = config.clone();
    let _ = app.emit("config-changed", &config);
    Ok(())
}

#[tauri::command]
pub fn get_monitors() -> Vec<MonitorInfo> {
    tracker::platform::enumerate_monitors()
}

#[tauri::command]
pub fn get_active_monitor() -> Option<MonitorInfo> {
    tracker::platform::get_active_monitor()
}
