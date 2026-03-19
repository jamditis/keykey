mod config;
mod commands;
mod keyboard;

use tauri::{
    Emitter, Manager,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let toggle_i = MenuItem::with_id(app, "toggle", "Pause capture", true, None::<&str>)?;
    let switch_mode_i = MenuItem::with_id(app, "switch_mode", "Switch to visual keyboard", true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&toggle_i, &switch_mode_i, &settings_i, &separator, &quit_i])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("KeyKey - active")
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "toggle" => {
                    if let Some(overlay) = app.get_webview_window("overlay") {
                        if overlay.is_visible().unwrap_or(false) {
                            let _ = overlay.hide();
                            let _ = app.emit("capture-toggled", false);
                        } else {
                            let _ = overlay.show();
                            let _ = app.emit("capture-toggled", true);
                        }
                    }
                }
                "switch_mode" => {
                    let _ = app.emit("switch-display-mode", ());
                }
                "settings" => {
                    use tauri::WebviewUrl;
                    if let Some(existing) = app.get_webview_window("settings") {
                        let _ = existing.show();
                        let _ = existing.set_focus();
                    } else {
                        let _ = tauri::WebviewWindowBuilder::new(
                            app,
                            "settings",
                            WebviewUrl::App("/".into()),
                        )
                        .title("KeyKey settings")
                        .inner_size(700.0, 500.0)
                        .build();
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(win) = app.get_webview_window("settings") {
                    if win.is_visible().unwrap_or(false) {
                        let _ = win.hide();
                    } else {
                        let _ = win.show();
                        let _ = win.set_focus();
                    }
                } else {
                    use tauri::WebviewUrl;
                    let _ = tauri::WebviewWindowBuilder::new(
                        app,
                        "settings",
                        WebviewUrl::App("/".into()),
                    )
                    .title("KeyKey settings")
                    .inner_size(700.0, 500.0)
                    .build();
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(commands::ConfigState(std::sync::Mutex::new(
            config::store::load_config(),
        )))
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config
        ])
        .setup(|app| {
            if let Some(overlay) = app.get_webview_window("overlay") {
                overlay.set_ignore_cursor_events(true)?;
            }

            setup_tray(app)?;
            keyboard::listener::start_listener(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
