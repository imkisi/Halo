use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub label: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default = "default_type")]
    pub r#type: String,
    pub value: String,
    #[serde(default = "default_category")]
    pub category: String,
}

fn default_type() -> String {
    "hotkey".to_string()
}

fn default_category() -> String {
    "SYSTEM".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppProfile {
    pub id: String,
    pub name: String,
    pub executable: String,
    #[serde(default = "default_app_icon")]
    pub icon: String,
    pub actions: Vec<ActionItem>,
}

fn default_app_icon() -> String {
    "default".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomShortcut {
    pub label: String,
    pub value: String,
    #[serde(default = "default_custom_category")]
    pub category: String,
}

fn default_custom_category() -> String {
    "CUSTOM".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_radius")]
    pub base_radius: u32,
    #[serde(default = "default_active_app")]
    pub active_app_id: String,
    #[serde(default)]
    pub apps: Vec<AppProfile>,
    #[serde(default)]
    pub custom_shortcuts: Vec<CustomShortcut>,
}

fn default_theme() -> String {
    "light".to_string()
}

fn default_radius() -> u32 {
    120
}

fn default_active_app() -> String {
    "default".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            base_radius: default_radius(),
            active_app_id: default_active_app(),
            apps: get_default_apps(),
            custom_shortcuts: vec![CustomShortcut {
                label: "Cmd+Shift+=".to_string(),
                value: "ctrl+shift+=".to_string(),
                category: "CUSTOM".to_string(),
            }],
        }
    }
}

pub fn get_default_apps() -> Vec<AppProfile> {
    vec![
        AppProfile {
            id: "default".to_string(),
            name: "Default".to_string(),
            executable: "*".to_string(),
            icon: "default".to_string(),
            actions: vec![
                ActionItem {
                    label: "Play/Pause".to_string(),
                    icon: "play.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "playpause".to_string(),
                    category: "MEDIA & VOLUME".to_string(),
                },
                ActionItem {
                    label: "New Note".to_string(),
                    icon: "note.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "win+n".to_string(),
                    category: "SYSTEM".to_string(),
                },
                ActionItem {
                    label: "Task View".to_string(),
                    icon: "taskview.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "win+tab".to_string(),
                    category: "NAVIGATION".to_string(),
                },
                ActionItem {
                    label: "Lock Workstation".to_string(),
                    icon: "lock.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "win+l".to_string(),
                    category: "SYSTEM".to_string(),
                },
                ActionItem {
                    label: "Halo Settings".to_string(),
                    icon: "app_icon.png".to_string(),
                    r#type: "internal".to_string(),
                    value: "open_settings".to_string(),
                    category: "SYSTEM".to_string(),
                },
                ActionItem {
                    label: "Screenshot".to_string(),
                    icon: "screenshot.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "win+shift+s".to_string(),
                    category: "NAVIGATION".to_string(),
                },
                ActionItem {
                    label: "Emoji Panel".to_string(),
                    icon: "emoji.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "win+.".to_string(),
                    category: "KEYBOARD".to_string(),
                },
                ActionItem {
                    label: "File Explorer".to_string(),
                    icon: "folder.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "win+e".to_string(),
                    category: "OPEN".to_string(),
                },
            ],
        },
        AppProfile {
            id: "photoshop".to_string(),
            name: "Photoshop".to_string(),
            executable: "Photoshop.exe".to_string(),
            icon: "photoshop".to_string(),
            actions: vec![
                ActionItem {
                    label: "Play/Pause".to_string(),
                    icon: "play.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "space".to_string(),
                    category: "MEDIA & VOLUME".to_string(),
                },
                ActionItem {
                    label: "New Layer".to_string(),
                    icon: "note.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "ctrl+shift+n".to_string(),
                    category: "SYSTEM".to_string(),
                },
                ActionItem {
                    label: "Free Transform".to_string(),
                    icon: "taskview.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "ctrl+t".to_string(),
                    category: "NAVIGATION".to_string(),
                },
                ActionItem {
                    label: "Deselect".to_string(),
                    icon: "lock.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "ctrl+d".to_string(),
                    category: "SYSTEM".to_string(),
                },
                ActionItem {
                    label: "Halo Settings".to_string(),
                    icon: "app_icon.png".to_string(),
                    r#type: "internal".to_string(),
                    value: "open_settings".to_string(),
                    category: "SYSTEM".to_string(),
                },
                ActionItem {
                    label: "Export As".to_string(),
                    icon: "screenshot.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "ctrl+alt+shift+w".to_string(),
                    category: "NAVIGATION".to_string(),
                },
                ActionItem {
                    label: "Brush Tool".to_string(),
                    icon: "emoji.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "b".to_string(),
                    category: "KEYBOARD".to_string(),
                },
                ActionItem {
                    label: "File Explorer".to_string(),
                    icon: "folder.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "win+e".to_string(),
                    category: "OPEN".to_string(),
                },
            ],
        },
        AppProfile {
            id: "aftereffects".to_string(),
            name: "After Effects".to_string(),
            executable: "AfterFX.exe".to_string(),
            icon: "aftereffects".to_string(),
            actions: vec![
                ActionItem {
                    label: "Play/Pause RAM".to_string(),
                    icon: "play.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "space".to_string(),
                    category: "MEDIA & VOLUME".to_string(),
                },
                ActionItem {
                    label: "Split Layer".to_string(),
                    icon: "note.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "ctrl+shift+d".to_string(),
                    category: "SYSTEM".to_string(),
                },
                ActionItem {
                    label: "Easy Ease".to_string(),
                    icon: "taskview.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "f9".to_string(),
                    category: "NAVIGATION".to_string(),
                },
                ActionItem {
                    label: "Keyframe Velocity".to_string(),
                    icon: "lock.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "ctrl+shift+k".to_string(),
                    category: "SYSTEM".to_string(),
                },
                ActionItem {
                    label: "Halo Settings".to_string(),
                    icon: "app_icon.png".to_string(),
                    r#type: "internal".to_string(),
                    value: "open_settings".to_string(),
                    category: "SYSTEM".to_string(),
                },
                ActionItem {
                    label: "Composition".to_string(),
                    icon: "screenshot.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "ctrl+k".to_string(),
                    category: "NAVIGATION".to_string(),
                },
                ActionItem {
                    label: "Render Queue".to_string(),
                    icon: "folder.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "ctrl+m".to_string(),
                    category: "OPEN".to_string(),
                },
                ActionItem {
                    label: "Fit to Comp".to_string(),
                    icon: "desktop.svg".to_string(),
                    r#type: "hotkey".to_string(),
                    value: "ctrl+alt+f".to_string(),
                    category: "SYSTEM".to_string(),
                },
            ],
        },
    ]
}

pub fn get_config_path() -> PathBuf {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let p = parent.join("config.json");
            if p.exists() {
                return p;
            }
        }
    }
    PathBuf::from("config.json")
}

pub fn load_config() -> AppConfig {
    let path = get_config_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            // First attempt to deserialize full AppConfig
            if let Ok(mut config) = serde_json::from_str::<AppConfig>(&content) {
                if config.apps.is_empty() {
                    config.apps = get_default_apps();
                }
                return config;
            }
            // Backward-compatibility: if older format with direct "actions"
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                let theme = val.get("theme").and_then(|v| v.as_str()).unwrap_or("light").to_string();
                let base_radius = val.get("base_radius").and_then(|v| v.as_u64()).unwrap_or(120) as u32;
                let mut actions = Vec::new();
                if let Some(act_arr) = val.get("actions").and_then(|v| v.as_array()) {
                    for a in act_arr {
                        if let Ok(item) = serde_json::from_value::<ActionItem>(a.clone()) {
                            actions.push(item);
                        }
                    }
                }
                let mut default_apps = get_default_apps();
                if !actions.is_empty() {
                    default_apps[0].actions = actions;
                }
                return AppConfig {
                    theme,
                    base_radius,
                    active_app_id: "default".to_string(),
                    apps: default_apps,
                    custom_shortcuts: vec![CustomShortcut {
                        label: "Cmd+Shift+=".to_string(),
                        value: "ctrl+shift+=".to_string(),
                        category: "CUSTOM".to_string(),
                    }],
                };
            }
        }
    }

    let default_cfg = AppConfig::default();
    let _ = save_config(&default_cfg);
    default_cfg
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = get_config_path();
    let json_str = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, json_str).map_err(|e| e.to_string())
}

impl AppConfig {
    pub fn get_profile_for_executable(&self, exe_name: &str) -> &AppProfile {
        let exe_lower = exe_name.to_lowercase();
        for app in &self.apps {
            if app.executable == "*" {
                continue;
            }
            if exe_lower.contains(&app.executable.to_lowercase()) {
                return app;
            }
        }
        // Fallback to active app profile or default
        if let Some(active) = self.apps.iter().find(|a| a.id == self.active_app_id) {
            return active;
        }
        &self.apps[0]
    }

    pub fn get_profile_by_id(&self, id: &str) -> Option<&AppProfile> {
        self.apps.iter().find(|a| a.id == id)
    }

    pub fn get_profile_by_id_mut(&mut self, id: &str) -> Option<&mut AppProfile> {
        self.apps.iter_mut().find(|a| a.id == id)
    }
}
