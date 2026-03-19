use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayMode {
    TextStream,
    VisualKeyboard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionStrategy {
    FollowActiveWindow,
    FollowMouse,
    Pinned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StackDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierMode {
    Smart,
    AlwaysShow,
    NeverShow,
    ComboOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub mode: DisplayMode,
    pub position_strategy: PositionStrategy,
    pub corner: Corner,
    pub margins: Margins,
    pub max_visible: u32,
    pub fade_duration_ms: u64,
    pub stack_direction: StackDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierOverrides {
    pub ctrl: ModifierMode,
    pub alt: ModifierMode,
    pub shift: ModifierMode,
    pub win: ModifierMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub smart_mode: bool,
    pub smart_threshold_ms: u64,
    pub repeat_window_ms: u64,
    pub show_all_keystrokes: bool,
    pub modifier_overrides: ModifierOverrides,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub active_theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub toggle_capture: Option<String>,
    pub switch_mode: Option<String>,
    pub toggle_overlay: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: u32,
    pub display: DisplayConfig,
    pub input: InputConfig,
    pub appearance: AppearanceConfig,
    pub shortcuts: ShortcutConfig,
    #[serde(default = "default_true")]
    pub first_launch: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            version: 1,
            display: DisplayConfig {
                mode: DisplayMode::TextStream,
                position_strategy: PositionStrategy::FollowActiveWindow,
                corner: Corner::BottomRight,
                margins: Margins { x: 24, y: 24 },
                max_visible: 5,
                fade_duration_ms: 2000,
                stack_direction: StackDirection::Up,
            },
            input: InputConfig {
                smart_mode: true,
                smart_threshold_ms: 200,
                repeat_window_ms: 500,
                show_all_keystrokes: false,
                modifier_overrides: ModifierOverrides {
                    ctrl: ModifierMode::ComboOnly,
                    alt: ModifierMode::ComboOnly,
                    shift: ModifierMode::Smart,
                    win: ModifierMode::AlwaysShow,
                },
            },
            appearance: AppearanceConfig {
                active_theme: "dark-glass".to_string(),
            },
            shortcuts: ShortcutConfig {
                toggle_capture: None,
                switch_mode: None,
                toggle_overlay: None,
            },
            first_launch: true,
        }
    }
}
