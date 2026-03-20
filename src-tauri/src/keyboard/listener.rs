use rdev::{listen, Event, EventType, Key};
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;
use tauri::{AppHandle, Emitter, Listener, Manager};
use super::processor::{EventProcessor, ModifierMode};
use crate::config::schema::{AppConfig, ModifierMode as CfgModifierMode};
use crate::commands::ConfigState;

/// Shared flag to pause/resume event emission without stopping the rdev hook.
static CAPTURE_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_capture_enabled(enabled: bool) {
    CAPTURE_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn toggle_capture() -> bool {
    let was_enabled = CAPTURE_ENABLED.load(Ordering::Relaxed);
    let new_state = !was_enabled;
    CAPTURE_ENABLED.store(new_state, Ordering::Relaxed);
    new_state
}

#[derive(Debug, Clone, Serialize)]
pub struct KeyEvent {
    pub key: String,
    pub key_code: String,
    pub event_type: String,
    pub timestamp: u64,
}

fn key_to_display_name(key: &Key, name: &Option<String>) -> String {
    match key {
        Key::ControlLeft | Key::ControlRight => "Ctrl".to_string(),
        Key::ShiftLeft | Key::ShiftRight => "Shift".to_string(),
        Key::Alt => "Alt".to_string(),
        Key::AltGr => "AltGr".to_string(),
        Key::MetaLeft | Key::MetaRight => "Win".to_string(),
        Key::Return => "Enter".to_string(),
        Key::Space => "Space".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::Escape => "Esc".to_string(),
        Key::Delete => "Del".to_string(),
        Key::UpArrow => "Up".to_string(),
        Key::DownArrow => "Down".to_string(),
        Key::LeftArrow => "Left".to_string(),
        Key::RightArrow => "Right".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PgUp".to_string(),
        Key::PageDown => "PgDn".to_string(),
        Key::CapsLock => "CapsLock".to_string(),
        Key::PrintScreen => "PrtSc".to_string(),
        Key::ScrollLock => "ScrLk".to_string(),
        Key::Pause => "Pause".to_string(),
        Key::NumLock => "NumLock".to_string(),
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),
        Key::Insert => "Ins".to_string(),
        Key::BackQuote => "`".to_string(),
        Key::Minus => "-".to_string(),
        Key::Equal => "=".to_string(),
        Key::LeftBracket => "[".to_string(),
        Key::RightBracket => "]".to_string(),
        Key::BackSlash => "\\".to_string(),
        Key::SemiColon => ";".to_string(),
        Key::Quote => "'".to_string(),
        Key::Comma => ",".to_string(),
        Key::Dot => ".".to_string(),
        Key::Slash => "/".to_string(),
        Key::IntlBackslash => "\\".to_string(),
        Key::KpMinus => "-".to_string(),
        Key::KpMultiply => "*".to_string(),
        Key::KpDivide => "/".to_string(),
        Key::KpPlus => "+".to_string(),
        Key::KpReturn => "Enter".to_string(),
        _ => {
            // Log unhandled keys to file so we can add explicit mappings (debug builds only)
            if cfg!(debug_assertions) {
                if let Ok(mut f) = std::fs::OpenOptions::new()
                    .create(true).append(true)
                    .open(std::env::temp_dir().join("keykey-debug.log"))
                {
                    use std::io::Write;
                    let _ = writeln!(f, "unhandled key: {:?}, name: {:?}", key, name);
                }
            }

            // Use the OS-provided name if it's a printable character
            if let Some(n) = name {
                let printable: String = n.chars().filter(|c| !c.is_control()).collect();
                if !printable.is_empty() {
                    return printable.to_uppercase();
                }
            }
            // Last resort: clean up the debug format
            let debug = format!("{:?}", key);
            // Strip "Key" prefix and "Unknown()" wrapper
            if debug.starts_with("Unknown(") {
                format!("Key{}", &debug[8..debug.len()-1])
            } else {
                debug
            }
        }
    }
}

pub fn key_to_code(key: &Key) -> String {
    format!("{:?}", key)
}

fn convert_modifier_mode(mode: &CfgModifierMode) -> ModifierMode {
    match mode {
        CfgModifierMode::Smart => ModifierMode::Smart,
        CfgModifierMode::AlwaysShow => ModifierMode::AlwaysShow,
        CfgModifierMode::NeverShow => ModifierMode::NeverShow,
        CfgModifierMode::ComboOnly => ModifierMode::ComboOnly,
    }
}

fn apply_config_to_processor(processor: &mut EventProcessor, config: &AppConfig) {
    processor.smart_threshold_ms = config.input.smart_threshold_ms;
    processor.show_all_keystrokes = config.input.show_all_keystrokes;
    processor.repeat_window_ms = config.input.repeat_window_ms;
    processor.set_modifier_mode("Ctrl", convert_modifier_mode(&config.input.modifier_overrides.ctrl));
    processor.set_modifier_mode("Alt", convert_modifier_mode(&config.input.modifier_overrides.alt));
    processor.set_modifier_mode("Shift", convert_modifier_mode(&config.input.modifier_overrides.shift));
    processor.set_modifier_mode("Win", convert_modifier_mode(&config.input.modifier_overrides.win));
}

pub fn start_listener(app_handle: AppHandle) {
    let (tx, rx) = mpsc::channel::<Event>();

    thread::spawn(move || {
        if let Err(e) = listen(move |event| {
            match &event.event_type {
                EventType::KeyPress(_) | EventType::KeyRelease(_) => {
                    let _ = tx.send(event);
                }
                _ => {}
            }
        }) {
            eprintln!("rdev listener error: {:?}", e);
        }
    });

    // Listen for capture-toggled events from tray/hotkeys
    let app_clone = app_handle.clone();
    app_clone.listen("capture-toggled", |event| {
        if let Ok(enabled) = event.payload().parse::<bool>() {
            set_capture_enabled(enabled);
        }
    });

    // Channel for config updates to processor thread
    let (config_tx, config_rx) = mpsc::channel::<AppConfig>();
    let app_clone2 = app_handle.clone();
    app_clone2.listen("config-changed", move |event| {
        if let Ok(config) = serde_json::from_str::<AppConfig>(event.payload()) {
            let _ = config_tx.send(config);
        }
    });

    // Read initial config to configure processor
    let initial_config = {
        let state = app_handle.state::<ConfigState>();
        let guard = state.0.lock().unwrap();
        guard.clone()
    };

    thread::spawn(move || {
        let mut processor = EventProcessor::new();
        apply_config_to_processor(&mut processor, &initial_config);

        while let Ok(event) = rx.recv() {
            // Skip event emission when capture is paused
            if !CAPTURE_ENABLED.load(Ordering::Relaxed) {
                continue;
            }

            // Apply any pending config updates (non-blocking)
            while let Ok(config) = config_rx.try_recv() {
                apply_config_to_processor(&mut processor, &config);
            }

            let (key, is_press) = match &event.event_type {
                EventType::KeyPress(k) => (k, true),
                EventType::KeyRelease(k) => (k, false),
                _ => continue,
            };

            let display_name = key_to_display_name(key, &event.name);

            // Debug: log every key press to file (debug builds only)
            if cfg!(debug_assertions) && is_press {
                if let Ok(mut f) = std::fs::OpenOptions::new()
                    .create(true).append(true)
                    .open(std::env::temp_dir().join("keykey-debug.log"))
                {
                    use std::io::Write;
                    let _ = writeln!(f, "key={:?} name={:?} display=\"{}\"", key, &event.name, &display_name);
                }
            }

            let timestamp = event
                .time
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;

            // Always emit raw key event (visual keyboard needs press/release)
            let raw_event = KeyEvent {
                key: display_name.clone(),
                key_code: key_to_code(key),
                event_type: if is_press { "press" } else { "release" }.to_string(),
                timestamp,
            };
            let _ = app_handle.emit("key-event", &raw_event);

            // Emit processed display event (text stream uses this)
            let display_event = if is_press {
                processor.on_key_press(&display_name, timestamp)
            } else {
                processor.on_key_release(&display_name, timestamp)
            };

            if let Some(evt) = display_event {
                let _ = app_handle.emit("display-event", &evt);
            }
        }
    });
}
