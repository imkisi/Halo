use eframe::egui::{
    self, Align2, Color32, FontId, Pos2, Rect, Rounding, Stroke, Vec2,
};
use std::f32::consts::PI;

use crate::config::{save_config, ActionItem, AppConfig, AppProfile};
use crate::process::{get_running_apps, RunningApp};

pub struct SettingsState {
    pub selected_app_id: String,
    pub selected_slot_index: Option<usize>,
    pub show_add_app_modal: bool,
    pub show_shortcut_modal: bool,
    pub running_apps: Vec<RunningApp>,
    pub search_query: String,
    pub selected_running_app: Option<RunningApp>,
    pub recorded_keys: String,
}

impl SettingsState {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            selected_app_id: config.active_app_id.clone(),
            selected_slot_index: None,
            show_add_app_modal: false,
            show_shortcut_modal: false,
            running_apps: Vec::new(),
            search_query: String::new(),
            selected_running_app: None,
            recorded_keys: "Cmd + Shift + =".to_string(),
        }
    }
}

pub fn render_settings_ui(
    ui: &mut egui::Ui,
    config: &mut AppConfig,
    state: &mut SettingsState,
) {
    let is_dark = config.theme == "dark";
    let accent_blue = Color32::from_rgb(16, 96, 255);

    // 1. Top Bar: App Profile Tabs
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let mut app_to_delete: Option<String> = None;

        for app in &config.apps {
            let is_active = app.id == state.selected_app_id;
            let btn_text = if app.id == "default" {
                "Halo (Default)"
            } else {
                app.name.as_str()
            };

            let btn = egui::Button::new(btn_text)
                .fill(if is_active { accent_blue } else { ui.visuals().window_fill() })
                .rounding(8.0);

            if ui.add(btn).clicked() {
                state.selected_app_id = app.id.clone();
            }

            if app.id != "default" && is_active {
                if ui.button("✕").on_hover_text("Delete Profile").clicked() {
                    app_to_delete = Some(app.id.clone());
                }
            }
        }

        if let Some(del_id) = app_to_delete {
            config.apps.retain(|a| a.id != del_id);
            state.selected_app_id = "default".to_string();
            let _ = save_config(config);
        }

        if ui.button("+ Add Profile").clicked() {
            state.running_apps = get_running_apps();
            state.show_add_app_modal = true;
            state.search_query.clear();
            state.selected_running_app = None;
        }
    });

    ui.separator();

    // 2. Main Body: Left Sidebar + Center Ring Preview
    ui.horizontal(|ui| {
        // Left Column: Available Actions Palette
        ui.vertical(|ui| {
            ui.set_width(260.0);
            ui.heading("Actions Library");
            ui.label("Click a slot in the preview, then click an action to assign it:");
            ui.add_space(4.0);

            egui::ScrollArea::vertical().max_height(460.0).show(ui, |ui| {
                let categories = [
                    ("SYSTEM", vec![
                        ("New Note", "note.svg", "win+n"),
                        ("Lock Workstation", "lock.svg", "win+l"),
                        ("Show Desktop", "desktop.svg", "win+d"),
                        ("Task Manager", "taskview.svg", "ctrl+shift+esc"),
                    ]),
                    ("MEDIA & VOLUME", vec![
                        ("Play Pause", "play.svg", "playpause"),
                        ("Previous", "previous.svg", "prevtrack"),
                        ("Next", "next.svg", "nexttrack"),
                        ("Silent", "mute.svg", "volumemute"),
                        ("Volume Up", "volumeup.svg", "volumeup"),
                        ("Volume Down", "volumedown.svg", "volumedown"),
                    ]),
                    ("NAVIGATION", vec![
                        ("Screenshot", "screenshot.svg", "win+shift+s"),
                        ("Task View", "taskview.svg", "win+tab"),
                    ]),
                    ("TOOLS & KEYBOARD", vec![
                        ("File Explorer", "folder.svg", "win+e"),
                        ("Emoji Panel", "emoji.svg", "win+."),
                        ("Clipboard History", "clipboard.svg", "win+v"),
                        ("Paste Plain Text", "paste.svg", "ctrl+shift+v"),
                    ]),
                ];

                for (cat_name, items) in categories {
                    ui.collapsing(cat_name, |ui| {
                        for (label, icon, val) in items {
                            if ui.button(format!("{} {}", icon_glyph(icon), label)).clicked() {
                                if let Some(slot_idx) = state.selected_slot_index {
                                    if let Some(profile) = config.get_profile_by_id_mut(&state.selected_app_id) {
                                        if slot_idx < profile.actions.len() {
                                            profile.actions[slot_idx] = ActionItem {
                                                label: label.to_string(),
                                                icon: icon.to_string(),
                                                r#type: "hotkey".to_string(),
                                                value: val.to_string(),
                                                category: cat_name.to_string(),
                                            };
                                            let _ = save_config(config);
                                        }
                                    }
                                }
                            }
                        }
                    });
                }

                ui.collapsing("CUSTOM SHORTCUT", |ui| {
                    if ui.button("⚡ Assign Custom Shortcut").clicked() {
                        state.show_shortcut_modal = true;
                    }
                });
            });
        });

        ui.separator();

        // Right / Center Column: Preview Ring + Controls
        ui.vertical(|ui| {
            ui.heading("Radial Ring Preview");

            let current_profile = config.get_profile_by_id(&state.selected_app_id)
                .cloned()
                .unwrap_or_else(|| config.apps[0].clone());

            let (response, painter) = ui.allocate_painter(Vec2::new(420.0, 400.0), egui::Sense::click());
            let center = response.rect.center();
            let base_radius = config.base_radius as f32;

            // Draw center node
            let center_bg = if is_dark { Color32::from_rgb(40, 40, 40) } else { Color32::from_rgb(230, 230, 230) };
            painter.circle_filled(center, 18.0, center_bg);
            painter.text(center, Align2::CENTER_CENTER, "✕", FontId::proportional(14.0), Color32::GRAY);

            let actions = &current_profile.actions;
            let total = actions.len();
            let slot_radius = 24.0;

            for (idx, act) in actions.iter().enumerate() {
                let angle_deg = (idx as f32 * (360.0 / total as f32)) - 90.0;
                let angle_rad = angle_deg * (PI / 180.0);
                let slot_pos = Pos2::new(
                    center.x + base_radius * angle_rad.cos(),
                    center.y + base_radius * angle_rad.sin(),
                );

                let is_selected = state.selected_slot_index == Some(idx);
                let is_hovered = response.hover_pos().map_or(false, |p| p.distance(slot_pos) <= slot_radius);

                if response.clicked() && is_hovered {
                    state.selected_slot_index = Some(idx);
                }

                let slot_bg = if is_selected {
                    Color32::from_rgb(20, 40, 80)
                } else if is_dark {
                    Color32::from_rgb(26, 26, 26)
                } else {
                    Color32::WHITE
                };

                let stroke_color = if is_selected || is_hovered { accent_blue } else { Color32::GRAY };
                painter.circle_filled(slot_pos, slot_radius, slot_bg);
                painter.circle_stroke(slot_pos, slot_radius, Stroke::new(if is_selected { 3.0 } else { 1.5 }, stroke_color));

                let glyph = icon_glyph(&act.icon);
                painter.text(slot_pos, Align2::CENTER_CENTER, glyph, FontId::proportional(16.0), if is_selected { accent_blue } else { Color32::WHITE });

                // Draw label below/above
                let label_pos = Pos2::new(slot_pos.x, slot_pos.y + slot_radius + 10.0);
                painter.text(label_pos, Align2::CENTER_CENTER, &act.label, FontId::proportional(10.0), Color32::GRAY);
            }

            ui.add_space(8.0);

            // Bottom Controls: Theme & Radius
            ui.horizontal(|ui| {
                ui.label("Theme:");
                if ui.selectable_label(config.theme == "light", "Light").clicked() {
                    config.theme = "light".to_string();
                    let _ = save_config(config);
                }
                if ui.selectable_label(config.theme == "dark", "Dark").clicked() {
                    config.theme = "dark".to_string();
                    let _ = save_config(config);
                }

                ui.add_space(20.0);
                ui.label("Ring Radius:");
                let mut radius = config.base_radius as i32;
                if ui.add(egui::Slider::new(&mut radius, 80..=180).text("px")).changed() {
                    config.base_radius = radius as u32;
                    let _ = save_config(config);
                }
            });
        });
    });

    // 3. Modal: Add App Profile
    if state.show_add_app_modal {
        egui::Window::new("Add Per-App Shortcut Profile")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.set_width(360.0);
                ui.label("Select a running application or browse an .exe:");

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut state.search_query);
                    if ui.button("Browse .exe").clicked() {
                        if let Some(file) = rfd::FileDialog::new().add_filter("Executables", &["exe"]).pick_file() {
                            if let Some(file_name) = file.file_name().and_then(|f| f.to_str()) {
                                let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or(file_name);
                                state.selected_running_app = Some(RunningApp {
                                    name: stem.to_string(),
                                    executable: file_name.to_string(),
                                    title: stem.to_string(),
                                });
                            }
                        }
                    }
                });

                ui.add_space(8.0);

                let query = state.search_query.to_lowercase();
                egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
                    for app in &state.running_apps {
                        if query.is_empty() || app.name.to_lowercase().contains(&query) || app.executable.to_lowercase().contains(&query) {
                            let is_sel = state.selected_running_app.as_ref().map_or(false, |a| a.executable == app.executable);
                            if ui.selectable_label(is_sel, format!("{} ({})", app.name, app.executable)).clicked() {
                                state.selected_running_app = Some(app.clone());
                            }
                        }
                    }
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Add Profile").clicked() {
                        if let Some(selected) = &state.selected_running_app {
                            let id = selected.executable.replace(".exe", "").to_lowercase();
                            if !config.apps.iter().any(|a| a.id == id) {
                                let default_actions = config.apps[0].actions.clone();
                                config.apps.push(AppProfile {
                                    id: id.clone(),
                                    name: selected.name.clone(),
                                    executable: selected.executable.clone(),
                                    icon: id.clone(),
                                    actions: default_actions,
                                });
                                state.selected_app_id = id;
                                let _ = save_config(config);
                            }
                        }
                        state.show_add_app_modal = false;
                    }
                    if ui.button("Cancel").clicked() {
                        state.show_add_app_modal = false;
                    }
                });
            });
    }

    // 4. Modal: Custom Shortcut Recorder
    if state.show_shortcut_modal {
        egui::Window::new("Custom Shortcut")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.set_width(300.0);
                ui.label("Enter hotkey combination (e.g. ctrl+shift+a, win+v):");
                ui.text_edit_singleline(&mut state.recorded_keys);

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Save to Selected Slot").clicked() {
                        if let Some(slot_idx) = state.selected_slot_index {
                            if let Some(profile) = config.get_profile_by_id_mut(&state.selected_app_id) {
                                if slot_idx < profile.actions.len() {
                                    profile.actions[slot_idx] = ActionItem {
                                        label: state.recorded_keys.clone(),
                                        icon: "play.svg".to_string(),
                                        r#type: "hotkey".to_string(),
                                        value: state.recorded_keys.to_lowercase().replace(' ', ""),
                                        category: "CUSTOM".to_string(),
                                    };
                                    let _ = save_config(config);
                                }
                            }
                        }
                        state.show_shortcut_modal = false;
                    }
                    if ui.button("Cancel").clicked() {
                        state.show_shortcut_modal = false;
                    }
                });
            });
    }
}

fn icon_glyph(icon: &str) -> &'static str {
    match icon {
        "play.svg" | "play" => "▶",
        "note.svg" | "note" => "📝",
        "taskview.svg" | "taskview" => "🗔",
        "lock.svg" | "lock" => "🔒",
        "screenshot.svg" | "screenshot" => "✂",
        "emoji.svg" | "emoji" => "☺",
        "folder.svg" | "folder" => "📁",
        "desktop.svg" | "desktop" => "🖥",
        "clipboard.svg" | "clipboard" => "📋",
        "paste.svg" | "paste" => "📄",
        "mute.svg" | "mute" | "silent.svg" | "silent" => "🔇",
        "previous.svg" | "previous" => "⏮",
        "next.svg" | "next" => "⏭",
        "volumeup.svg" | "volumeup" => "🔊",
        "volumedown.svg" | "volumedown" => "🔉",
        _ => "●",
    }
}

