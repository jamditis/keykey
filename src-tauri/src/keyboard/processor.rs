use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone, Serialize)]
pub struct DisplayEvent {
    pub label: String,
    pub is_combo: bool,
    pub id: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModifierMode {
    Smart,
    AlwaysShow,
    NeverShow,
    ComboOnly,
}

pub struct EventProcessor {
    held_modifiers: HashSet<String>,
    modifier_press_times: HashMap<String, Instant>,
    modifier_used_in_combo: HashSet<String>,
    pub smart_threshold_ms: u64,
    pub show_all_keystrokes: bool,
    modifier_modes: HashMap<String, ModifierMode>,
    last_key: Option<String>,
    last_key_time: Option<u64>,
    repeat_count: u32,
    repeat_window_ms: u64,
    next_id: u32,
}

const MODIFIERS: &[&str] = &["Ctrl", "Alt", "Shift", "Win", "AltGr"];

// Canonical order for display: Ctrl, Alt, Shift, Win
const CANONICAL_ORDER: &[&str] = &["Ctrl", "Alt", "Shift", "Win", "AltGr"];

fn is_modifier(key: &str) -> bool {
    MODIFIERS.contains(&key)
}

impl EventProcessor {
    pub fn new() -> Self {
        let mut modifier_modes = HashMap::new();
        for &m in MODIFIERS {
            modifier_modes.insert(m.to_string(), ModifierMode::Smart);
        }

        Self {
            held_modifiers: HashSet::new(),
            modifier_press_times: HashMap::new(),
            modifier_used_in_combo: HashSet::new(),
            smart_threshold_ms: 200,
            show_all_keystrokes: true,
            modifier_modes,
            last_key: None,
            last_key_time: None,
            repeat_count: 0,
            repeat_window_ms: 500,
            next_id: 0,
        }
    }

    fn next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }

    pub fn set_modifier_mode(&mut self, modifier: &str, mode: ModifierMode) {
        self.modifier_modes.insert(modifier.to_string(), mode);
    }

    pub fn on_key_press(&mut self, key: &str, timestamp: u64) -> Option<DisplayEvent> {
        if is_modifier(key) {
            let mode = self.modifier_modes.get(key).cloned().unwrap_or(ModifierMode::Smart);
            self.held_modifiers.insert(key.to_string());
            self.modifier_press_times.insert(key.to_string(), Instant::now());

            if mode == ModifierMode::AlwaysShow {
                let id = self.next_id();
                return Some(DisplayEvent {
                    label: key.to_string(),
                    is_combo: false,
                    id,
                    timestamp,
                });
            }
            return None;
        }

        // Non-modifier key: build label from held modifiers in canonical order
        let active_modifiers: Vec<&str> = CANONICAL_ORDER
            .iter()
            .filter(|&&m| {
                self.held_modifiers.contains(m) && {
                    let mode = self.modifier_modes.get(m).cloned().unwrap_or(ModifierMode::Smart);
                    mode != ModifierMode::NeverShow
                }
            })
            .map(|&m| m)
            .collect();

        // Mark modifiers as used in combo
        for m in &active_modifiers {
            self.modifier_used_in_combo.insert(m.to_string());
        }

        let is_combo = !active_modifiers.is_empty();

        // If not showing all keystrokes and no modifiers, suppress
        if !self.show_all_keystrokes && !is_combo {
            return None;
        }

        let label = if is_combo {
            let mut parts: Vec<String> = active_modifiers.iter().map(|m| m.to_string()).collect();
            parts.push(key.to_string());
            parts.join(" + ")
        } else {
            key.to_string()
        };

        // Repeat detection
        let is_repeat = match (&self.last_key, self.last_key_time) {
            (Some(last), Some(last_time)) => {
                last == &label && timestamp.saturating_sub(last_time) <= self.repeat_window_ms
            }
            _ => false,
        };

        if is_repeat {
            self.repeat_count += 1;
        } else {
            self.repeat_count = 1;
        }

        self.last_key = Some(label.clone());
        self.last_key_time = Some(timestamp);

        let display_label = if self.repeat_count > 1 {
            format!("{} x{}", label, self.repeat_count)
        } else {
            label
        };

        let id = self.next_id();
        Some(DisplayEvent {
            label: display_label,
            is_combo,
            id,
            timestamp,
        })
    }

    pub fn on_key_release(&mut self, key: &str, timestamp: u64) -> Option<DisplayEvent> {
        if !is_modifier(key) {
            return None;
        }

        self.held_modifiers.remove(key);

        let mode = self.modifier_modes.get(key).cloned().unwrap_or(ModifierMode::Smart);

        match mode {
            ModifierMode::NeverShow | ModifierMode::ComboOnly | ModifierMode::AlwaysShow => {
                self.modifier_press_times.remove(key);
                self.modifier_used_in_combo.remove(key);
                None
            }
            ModifierMode::Smart => {
                let used_in_combo = self.modifier_used_in_combo.remove(key);
                if used_in_combo {
                    self.modifier_press_times.remove(key);
                    return None;
                }

                let held_duration = self
                    .modifier_press_times
                    .remove(key)
                    .map(|t| t.elapsed().as_millis() as u64)
                    .unwrap_or(u64::MAX);

                if held_duration <= self.smart_threshold_ms {
                    let id = self.next_id();
                    Some(DisplayEvent {
                        label: key.to_string(),
                        is_combo: false,
                        id,
                        timestamp,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(n: u64) -> u64 {
        n
    }

    #[test]
    fn test_regular_key_press() {
        let mut p = EventProcessor::new();
        let evt = p.on_key_press("A", ms(1000)).unwrap();
        assert_eq!(evt.label, "A");
        assert!(!evt.is_combo);
    }

    #[test]
    fn test_modifier_press_alone_produces_nothing() {
        let mut p = EventProcessor::new();
        let result = p.on_key_press("Ctrl", ms(1000));
        assert!(result.is_none());
    }

    #[test]
    fn test_combo_ctrl_c() {
        let mut p = EventProcessor::new();
        p.on_key_press("Ctrl", ms(1000));
        let evt = p.on_key_press("C", ms(1010)).unwrap();
        assert_eq!(evt.label, "Ctrl + C");
        assert!(evt.is_combo);
    }

    #[test]
    fn test_combo_ctrl_shift_s() {
        let mut p = EventProcessor::new();
        p.on_key_press("Ctrl", ms(1000));
        p.on_key_press("Shift", ms(1005));
        let evt = p.on_key_press("S", ms(1010)).unwrap();
        assert_eq!(evt.label, "Ctrl + Shift + S");
        assert!(evt.is_combo);
    }

    #[test]
    fn test_modifier_canonical_order() {
        let mut p = EventProcessor::new();
        // Press in non-canonical order: Win, Shift, Alt, Ctrl
        p.on_key_press("Win", ms(1000));
        p.on_key_press("Shift", ms(1001));
        p.on_key_press("Alt", ms(1002));
        p.on_key_press("Ctrl", ms(1003));
        let evt = p.on_key_press("X", ms(1010)).unwrap();
        assert_eq!(evt.label, "Ctrl + Alt + Shift + Win + X");
        assert!(evt.is_combo);
    }

    #[test]
    fn test_repeat_detection() {
        let mut p = EventProcessor::new();
        let e1 = p.on_key_press("A", ms(1000)).unwrap();
        assert_eq!(e1.label, "A");

        let e2 = p.on_key_press("A", ms(1100)).unwrap();
        assert_eq!(e2.label, "A x2");

        let e3 = p.on_key_press("A", ms(1200)).unwrap();
        assert_eq!(e3.label, "A x3");
    }

    #[test]
    fn test_different_key_breaks_repeat() {
        let mut p = EventProcessor::new();
        p.on_key_press("A", ms(1000)).unwrap();
        p.on_key_press("A", ms(1100)).unwrap();
        // Different key resets
        let e = p.on_key_press("B", ms(1200)).unwrap();
        assert_eq!(e.label, "B");
        // A again should start fresh
        let e2 = p.on_key_press("A", ms(1300)).unwrap();
        assert_eq!(e2.label, "A");
    }

    #[test]
    fn test_modifier_release_after_combo_no_display() {
        let mut p = EventProcessor::new();
        p.on_key_press("Ctrl", ms(1000));
        p.on_key_press("C", ms(1010));
        // Release Ctrl after combo — should not show
        let result = p.on_key_release("Ctrl", ms(1020));
        assert!(result.is_none());
    }

    #[test]
    fn test_modifier_never_show() {
        let mut p = EventProcessor::new();
        p.set_modifier_mode("Shift", ModifierMode::NeverShow);
        p.on_key_press("Shift", ms(1000));
        let evt = p.on_key_press("A", ms(1010)).unwrap();
        // Shift is NeverShow so it should not appear in label
        assert_eq!(evt.label, "A");
        assert!(!evt.is_combo);
    }

    #[test]
    fn test_modifier_always_show() {
        let mut p = EventProcessor::new();
        p.set_modifier_mode("Ctrl", ModifierMode::AlwaysShow);
        let evt = p.on_key_press("Ctrl", ms(1000)).unwrap();
        assert_eq!(evt.label, "Ctrl");
        assert!(!evt.is_combo);
    }

    #[test]
    fn test_modifier_combo_only() {
        let mut p = EventProcessor::new();
        p.set_modifier_mode("Ctrl", ModifierMode::ComboOnly);

        // Press alone — nothing on press
        let press_result = p.on_key_press("Ctrl", ms(1000));
        assert!(press_result.is_none());

        // Release alone — ComboOnly returns None on release too
        let release_result = p.on_key_release("Ctrl", ms(1050));
        assert!(release_result.is_none());

        // But Ctrl+C should work
        p.on_key_press("Ctrl", ms(2000));
        let combo_evt = p.on_key_press("C", ms(2010)).unwrap();
        assert_eq!(combo_evt.label, "Ctrl + C");
        assert!(combo_evt.is_combo);
    }

    #[test]
    fn test_show_all_keystrokes_false() {
        let mut p = EventProcessor::new();
        p.show_all_keystrokes = false;

        // Regular key suppressed
        let result = p.on_key_press("A", ms(1000));
        assert!(result.is_none());

        // Combo still shows
        p.on_key_press("Ctrl", ms(2000));
        let evt = p.on_key_press("S", ms(2010)).unwrap();
        assert_eq!(evt.label, "Ctrl + S");
        assert!(evt.is_combo);
    }
}
