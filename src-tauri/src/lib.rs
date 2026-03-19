mod config;
mod commands;
mod keyboard;
mod monitor;

use tauri::{
    Emitter, Manager,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

#[cfg(desktop)]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

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
            commands::save_config,
            commands::get_monitors,
            commands::get_active_monitor,
        ])
        .setup(|app| {
            if let Some(overlay) = app.get_webview_window("overlay") {
                overlay.set_ignore_cursor_events(true)?;
            }

            setup_tray(app)?;

            // Open settings on first launch, then clear the flag
            {
                let config = config::store::load_config();
                if config.first_launch {
                    use tauri::WebviewUrl;
                    let _ = tauri::WebviewWindowBuilder::new(
                        app,
                        "settings",
                        WebviewUrl::App("/".into()),
                    )
                    .title("KeyKey settings")
                    .inner_size(700.0, 500.0)
                    .build();

                    let mut updated = config;
                    updated.first_launch = false;
                    let _ = config::store::save_config(&updated);
                }
            }

            // Register global shortcuts from config
            #[cfg(desktop)]
            {
                let config = config::store::load_config();

                // Build a map of shortcut id -> action name so the handler can dispatch
                let mut shortcut_actions: std::collections::HashMap<u32, String> =
                    std::collections::HashMap::new();

                let mut shortcut_strings: Vec<String> = Vec::new();

                for (shortcut_str, action) in [
                    (&config.shortcuts.toggle_capture, "toggle_capture"),
                    (&config.shortcuts.switch_mode, "switch_mode"),
                    (&config.shortcuts.toggle_overlay, "toggle_overlay"),
                ] {
                    if let Some(s) = shortcut_str {
                        if let Ok(parsed) = s.parse::<tauri_plugin_global_shortcut::Shortcut>() {
                            shortcut_actions.insert(parsed.id(), action.to_string());
                            shortcut_strings.push(s.clone());
                        } else {
                            eprintln!("invalid shortcut string '{}', skipping", s);
                        }
                    }
                }

                let actions_for_handler = shortcut_actions;

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut, event| {
                            if event.state() == ShortcutState::Pressed {
                                if let Some(action) = actions_for_handler.get(&shortcut.id()) {
                                    match action.as_str() {
                                        "toggle_capture" | "toggle_overlay" => {
                                            if let Some(overlay) =
                                                app.get_webview_window("overlay")
                                            {
                                                let visible =
                                                    overlay.is_visible().unwrap_or(false);
                                                if visible {
                                                    let _ = overlay.hide();
                                                } else {
                                                    let _ = overlay.show();
                                                }
                                                let _ = app.emit("capture-toggled", !visible);
                                            }
                                        }
                                        "switch_mode" => {
                                            let _ = app.emit("switch-display-mode", ());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                // Register each shortcut
                for s in &shortcut_strings {
                    if let Ok(parsed) = s.parse::<tauri_plugin_global_shortcut::Shortcut>() {
                        if let Err(e) = app.global_shortcut().register(parsed) {
                            eprintln!("failed to register shortcut '{}': {}", s, e);
                        }
                    }
                }
            }

            keyboard::listener::start_listener(app.handle().clone());

            // Background thread: reposition overlay based on position strategy
            {
                use crate::config::schema::PositionStrategy;
                use crate::commands::ConfigState;

                let app_handle = app.handle().clone();
                std::thread::spawn(move || {
                    let mut last_monitor_name = String::new();
                    loop {
                        let strategy = {
                            let state = app_handle.state::<ConfigState>();
                            let config = state.0.lock().unwrap();
                            config.display.position_strategy.clone()
                        };

                        let target_monitor = match strategy {
                            PositionStrategy::FollowActiveWindow => {
                                monitor::tracker::platform::get_active_monitor()
                            }
                            PositionStrategy::FollowMouse => {
                                monitor::tracker::platform::get_cursor_monitor()
                            }
                            PositionStrategy::Pinned => {
                                std::thread::sleep(std::time::Duration::from_millis(500));
                                continue;
                            }
                        };

                        if let Some(m) = target_monitor {
                            if m.name != last_monitor_name {
                                last_monitor_name = m.name.clone();
                                if let Some(overlay) = app_handle.get_webview_window("overlay") {
                                    let x = m.work_x + m.work_width - 420;
                                    let y = m.work_y + m.work_height - 320;
                                    let _ = overlay.set_position(tauri::Position::Physical(
                                        tauri::PhysicalPosition::new(x, y),
                                    ));
                                }
                            }
                        }

                        std::thread::sleep(std::time::Duration::from_millis(250));
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
