use std::path::PathBuf;

use super::schema::AppConfig;

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("could not find system config dir")
        .join("keykey")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.json")
}

pub fn load_config() -> AppConfig {
    let path = config_path();

    if !path.exists() {
        let default = AppConfig::default();
        // best-effort write; if it fails we still return the default
        let _ = save_config(&default);
        return default;
    }

    let raw = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("config: read error: {e}, resetting to default");
            let default = AppConfig::default();
            let _ = save_config(&default);
            return default;
        }
    };

    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&raw);
    let value = match parsed {
        Ok(v) => v,
        Err(e) => {
            eprintln!("config: parse error: {e}, backing up and resetting");
            backup_corrupt(&raw);
            let default = AppConfig::default();
            let _ = save_config(&default);
            return default;
        }
    };

    // version check — bump this when a breaking migration is needed
    let file_version = value.get("version").and_then(|v| v.as_u64()).unwrap_or(0);
    let current_version = AppConfig::default().version as u64;

    if file_version < current_version {
        eprintln!(
            "config: version {file_version} < current {current_version}, migrating"
        );
        backup_versioned(&raw, file_version);
        // for now, migration = reset to default (extend this when needed)
        let default = AppConfig::default();
        let _ = save_config(&default);
        return default;
    }

    match serde_json::from_value::<AppConfig>(value) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("config: deserialize error: {e}, backing up and resetting");
            backup_corrupt(&raw);
            let default = AppConfig::default();
            let _ = save_config(&default);
            default
        }
    }
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let dir = config_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("could not create config dir: {e}"))?;

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("could not serialize config: {e}"))?;

    std::fs::write(config_path(), json)
        .map_err(|e| format!("could not write config: {e}"))?;

    Ok(())
}

fn backup_corrupt(raw: &str) {
    let backup = config_dir().join("config.corrupt.json");
    let _ = std::fs::create_dir_all(config_dir());
    let _ = std::fs::write(backup, raw);
}

fn backup_versioned(raw: &str, version: u64) {
    let backup = config_dir().join(format!("config.v{version}.bak.json"));
    let _ = std::fs::create_dir_all(config_dir());
    let _ = std::fs::write(backup, raw);
}

#[cfg(test)]
mod tests {
    use crate::config::schema::AppConfig;

    #[test]
    fn test_default_config_serializes() {
        let config = AppConfig::default();
        let json = serde_json::to_string_pretty(&config).expect("serialization failed");
        let roundtrip: AppConfig =
            serde_json::from_str(&json).expect("deserialization failed");
        // spot-check a few fields
        assert_eq!(roundtrip.version, config.version);
        assert_eq!(roundtrip.appearance.active_theme, config.appearance.active_theme);
        assert_eq!(roundtrip.input.smart_threshold_ms, config.input.smart_threshold_ms);
    }
}
