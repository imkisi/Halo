#![windows_subsystem = "windows"]

mod config;
mod hook;
mod input;
mod process;
mod ui;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use config::{load_config, AppConfig};
use hook::{start_mouse_hook, UserEvent};
use input::send_hotkey;
use ui::{render_radial_wheel, render_settings_ui, RadialResult, SettingsState};

use eframe::egui::{self, ViewportBuilder, ViewportCommand, ViewportId};
use image::GenericImageView;
use muda::{Menu, MenuItem};
use tray_icon::{TrayIcon, TrayIconBuilder};

fn load_tray_icon() -> Option<tray_icon::Icon> {
    let png_bytes = include_bytes!("../assets/app_icon.png");
    if let Ok(img) = image::load_from_memory(png_bytes) {
        let (width, height) = img.dimensions();
        let rgba = img.into_rgba8().into_raw();
        tray_icon::Icon::from_rgba(rgba, width, height).ok()
    } else {
        None
    }
}

struct HaloApp {
    config: AppConfig,
    settings_state: SettingsState,
    radial_visible: bool,
    settings_visible: bool,
    active_process: String,
    rx: Receiver<UserEvent>,
    tx: Sender<UserEvent>,
}

impl HaloApp {
    fn new(config: AppConfig, rx: Receiver<UserEvent>, tx: Sender<UserEvent>) -> Self {
        let settings_state = SettingsState::new(&config);
        Self {
            config,
            settings_state,
            radial_visible: false,
            settings_visible: false,
            active_process: String::new(),
            rx,
            tx,
        }
    }
}

impl eframe::App for HaloApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle incoming events from mouse hook and tray menu
        while let Ok(event) = self.rx.try_recv() {
            match event {
                UserEvent::ShowRadial { x, y, process_name } => {
                    self.radial_visible = true;
                    self.active_process = process_name;
                    let pos_x = (x - 260).max(0);
                    let pos_y = (y - 260).max(0);
                    ctx.send_viewport_cmd(ViewportCommand::OuterPosition(egui::pos2(pos_x as f32, pos_y as f32)));
                    ctx.send_viewport_cmd(ViewportCommand::Visible(true));
                    ctx.send_viewport_cmd(ViewportCommand::Focus);
                    ctx.request_repaint();
                }
                UserEvent::OpenSettings => {
                    self.settings_visible = true;
                    self.radial_visible = false;
                    ctx.send_viewport_cmd(ViewportCommand::Visible(false));
                    ctx.request_repaint();
                }
                UserEvent::CloseRadial => {
                    self.radial_visible = false;
                    ctx.send_viewport_cmd(ViewportCommand::Visible(false));
                }
                UserEvent::Quit => {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            }
        }

        // 1. Render Radial Overlay (Main Window)
        if self.radial_visible {
            let profile = self.config.get_profile_for_executable(&self.active_process).clone();
            let theme = self.config.theme.clone();
            let radius = self.config.base_radius as f32;

            egui::CentralPanel::default()
                .frame(egui::Frame::none())
                .show(ctx, |ui| {
                    let res = render_radial_wheel(ui, &profile, &theme, radius);
                    match res {
                        RadialResult::Execute(act) => {
                            self.radial_visible = false;
                            ctx.send_viewport_cmd(ViewportCommand::Visible(false));

                            if act.r#type == "internal" && act.value == "open_settings" {
                                self.settings_visible = true;
                            } else if act.r#type == "hotkey" {
                                let val_str = act.value.clone();
                                thread::spawn(move || {
                                    thread::sleep(Duration::from_millis(60));
                                    send_hotkey(&val_str);
                                });
                            }
                        }
                        RadialResult::Close => {
                            self.radial_visible = false;
                            ctx.send_viewport_cmd(ViewportCommand::Visible(false));
                        }
                        RadialResult::None => {}
                    }
                });

            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.radial_visible = false;
                ctx.send_viewport_cmd(ViewportCommand::Visible(false));
            }
        }

        // 2. Render Settings Window (Secondary Viewport)
        if self.settings_visible {
            ctx.show_viewport_immediate(
                ViewportId::from_hash_of("halo_settings_window"),
                ViewportBuilder::default()
                    .with_title("Halo Settings")
                    .with_inner_size([960.0, 640.0])
                    .with_resizable(true),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        render_settings_ui(ui, &mut self.config, &mut self.settings_state);
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.settings_visible = false;
                    }
                },
            );
        }

        // Keep polling if an active window is showing, otherwise sleep
        if self.radial_visible || self.settings_visible {
            ctx.request_repaint_after(Duration::from_millis(16));
        } else {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let (tx, rx) = channel::<UserEvent>();

    // Start background global mouse hook for double Middle Mouse Button clicks
    start_mouse_hook(tx.clone());

    // System Tray Menu setup
    let tray_menu = Menu::new();
    let item_settings = MenuItem::new("Halo Settings", true, None);
    let item_toggle = MenuItem::new("Toggle Radial Menu", true, None);
    let item_exit = MenuItem::new("Exit", true, None);
    let _ = tray_menu.append_items(&[&item_settings, &item_toggle, &item_exit]);

    let mut tray_builder = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu.clone()))
        .with_tooltip("Halo - Radial Shortcut Menu");

    if let Some(icon) = load_tray_icon() {
        tray_builder = tray_builder.with_icon(icon);
    }
    let _tray_icon: Option<TrayIcon> = tray_builder.build().ok();

    // Spawn tray event listener thread
    let tx_tray = tx.clone();
    let settings_id = item_settings.id();
    let toggle_id = item_toggle.id();
    let exit_id = item_exit.id();

    thread::spawn(move || {
        let menu_channel = muda::MenuEvent::receiver();
        while let Ok(event) = menu_channel.recv() {
            if event.id == settings_id {
                let _ = tx_tray.send(UserEvent::OpenSettings);
            } else if event.id == toggle_id {
                let _ = tx_tray.send(UserEvent::ShowRadial {
                    x: 600,
                    y: 400,
                    process_name: String::new(),
                });
            } else if event.id == exit_id {
                let _ = tx_tray.send(UserEvent::Quit);
                break;
            }
        }
    });

    let config = load_config();

    // Native Options for the Radial Overlay (Main Window)
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Halo Radial")
            .with_inner_size([520.0, 520.0])
            .with_transparent(true)
            .with_decorations(false)
            .with_always_on_top(true)
            .with_visible(false),
        ..Default::default()
    };

    eframe::run_native(
        "Halo",
        native_options,
        Box::new(move |_cc| Ok(Box::new(HaloApp::new(config, rx, tx)))),
    )
}
