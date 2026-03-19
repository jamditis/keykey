use rdev::{listen, Event, EventType, Key};
use serde::Serialize;
use std::sync::mpsc;
use std::thread;
use tauri::{AppHandle, Emitter};

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
        _ => {
            if let Some(n) = name {
                if !n.is_empty() {
                    return n.to_uppercase();
                }
            }
            format!("{:?}", key)
        }
    }
}

pub fn key_to_code(key: &Key) -> String {
    format!("{:?}", key)
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

    thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            let (key, event_type_str) = match &event.event_type {
                EventType::KeyPress(k) => (k, "press"),
                EventType::KeyRelease(k) => (k, "release"),
                _ => continue,
            };

            let key_event = KeyEvent {
                key: key_to_display_name(key, &event.name),
                key_code: key_to_code(key),
                event_type: event_type_str.to_string(),
                timestamp: event
                    .time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
            };

            let _ = app_handle.emit("key-event", &key_event);
        }
    });
}
