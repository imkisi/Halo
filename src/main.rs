#![windows_subsystem = "windows"]

mod config;
mod hook;
mod input;
mod process;
mod ui;

use std::sync::{Arc, Mutex};
use std::thread;

use config::{load_config, save_config, AppConfig};
use hook::{start_mouse_hook, UserEvent};
use input::send_hotkey;
use process::get_running_apps;
use ui::{get_radial_html, get_settings_html, APP_ICON_BASE64};

use image::GenericImageView;
use muda::{Menu, MenuItem};
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use tray_icon::{TrayIcon, TrayIconBuilder};
use wry::WebViewBuilder;

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

fn main() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let config_arc = Arc::new(Mutex::new(load_config()));

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

    // 1. Radial Overlay Window
    let radial_window = WindowBuilder::new()
        .with_title("Halo Radial")
        .with_inner_size(LogicalSize::new(520.0, 520.0))
        .with_transparent(true)
        .with_decorations(false)
        .with_always_on_top(true)
        .with_visible(false)
        .build(&event_loop)
        .expect("Failed to create radial window");

    let radial_proxy = proxy.clone();

    let radial_webview = WebViewBuilder::new()
        .with_transparent(true)
        .with_html(get_radial_html())
        .with_ipc_handler(move |req| {
            let body = req.body();
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
                let msg_type = val.get("type").and_then(|v| v.as_str()).unwrap_or("");
                match msg_type {
                    "execute" => {
                        let _ = radial_proxy.send_event(UserEvent::CloseRadial);
                        if let Some(act) = val.get("action") {
                            let act_type = act.get("type").and_then(|v| v.as_str()).unwrap_or("hotkey");
                            let act_val = act.get("value").and_then(|v| v.as_str()).unwrap_or("");

                            if act_type == "internal" && act_val == "open_settings" {
                                let _ = radial_proxy.send_event(UserEvent::OpenSettings);
                            } else if act_type == "hotkey" {
                                let val_str = act_val.to_string();
                                thread::spawn(move || {
                                    thread::sleep(std::time::Duration::from_millis(60));
                                    send_hotkey(&val_str);
                                });
                            }
                        }
                    }
                    "close" => {
                        let _ = radial_proxy.send_event(UserEvent::CloseRadial);
                    }
                    _ => {}
                }
            }
        })
        .build(&radial_window)
        .expect("Failed to create radial webview");

    // 2. Settings Window
    let settings_window = WindowBuilder::new()
        .with_title("Halo Settings")
        .with_inner_size(LogicalSize::new(980.0, 640.0))
        .with_decorations(false)
        .with_visible(false)
        .build(&event_loop)
        .expect("Failed to create settings window");

    let settings_proxy = proxy.clone();
    let settings_config = Arc::clone(&config_arc);

    let settings_webview = WebViewBuilder::new()
        .with_html(get_settings_html())
        .with_ipc_handler(move |req| {
            let body = req.body();
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
                let msg_type = val.get("type").and_then(|v| v.as_str()).unwrap_or("");
                match msg_type {
                    "save_config" => {
                        if let Some(cfg_val) = val.get("config") {
                            if let Ok(new_cfg) = serde_json::from_value::<AppConfig>(cfg_val.clone()) {
                                let _ = save_config(&new_cfg);
                                let mut c = settings_config.lock().unwrap();
                                *c = new_cfg;
                            }
                        }
                    }
                    "get_running_apps" => {
                        let _ = settings_proxy.send_event(UserEvent::ReloadRunningApps);
                    }
                    "browse_app" => {
                        let sp = settings_proxy.clone();
                        thread::spawn(move || {
                            if let Some(file) = rfd::FileDialog::new()
                                .add_filter("Applications", &["exe"])
                                .pick_file()
                            {
                                if let Some(file_name) = file.file_name().and_then(|f| f.to_str()) {
                                    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or(file_name);
                                    let app_obj = serde_json::json!({
                                        "name": stem,
                                        "executable": file_name,
                                        "title": stem
                                    });
                                    let _ = sp.send_event(UserEvent::AppFileSelected(app_obj.to_string()));
                                }
                            }
                        });
                    }
                    "minimize_window" => {
                        let _ = settings_proxy.send_event(UserEvent::MinimizeSettings);
                    }
                    "close_window" => {
                        let _ = settings_proxy.send_event(UserEvent::CloseSettings);
                    }
                    _ => {}
                }
            }
        })
        .build(&settings_window)
        .expect("Failed to create settings webview");

    let radial_window_id = radial_window.id();
    let settings_window_id = settings_window.id();

    // Start background global mouse hook for double Middle Mouse Button clicks
    start_mouse_hook(proxy.clone());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Check tray menu clicks
        if let Ok(menu_event) = muda::MenuEvent::receiver().try_recv() {
            if menu_event.id == item_settings.id() {
                let _ = proxy.send_event(UserEvent::OpenSettings);
            } else if menu_event.id == item_toggle.id() {
                let _ = proxy.send_event(UserEvent::ShowRadial {
                    x: 600,
                    y: 400,
                    process_name: String::new(),
                });
            } else if menu_event.id == item_exit.id() {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::UserEvent(user_event) => match user_event {
                UserEvent::ShowRadial { x, y, process_name } => {
                    let cfg = config_arc.lock().unwrap();
                    let profile = cfg.get_profile_for_executable(&process_name);

                    let payload = serde_json::json!({
                        "theme": cfg.theme,
                        "base_radius": cfg.base_radius,
                        "app_name": profile.name,
                        "actions": profile.actions,
                        "app_icon_base64": APP_ICON_BASE64,
                    });

                    let js = format!("window.updateRadial({});", payload);
                    let _ = radial_webview.evaluate_script(&js);

                    // Position radial overlay centered at cursor (x, y)
                    let pos_x = (x - 260).max(0);
                    let pos_y = (y - 260).max(0);
                    radial_window.set_outer_position(LogicalPosition::new(pos_x as f64, pos_y as f64));
                    radial_window.set_visible(true);
                    radial_window.set_focus();
                }
                UserEvent::OpenSettings => {
                    let cfg = config_arc.lock().unwrap();
                    let payload = serde_json::json!({
                        "theme": cfg.theme,
                        "base_radius": cfg.base_radius,
                        "active_app_id": cfg.active_app_id,
                        "apps": cfg.apps,
                        "custom_shortcuts": cfg.custom_shortcuts,
                        "app_icon_base64": APP_ICON_BASE64,
                    });

                    let js = format!("window.loadSettingsData({});", payload);
                    let _ = settings_webview.evaluate_script(&js);

                    settings_window.set_visible(true);
                    settings_window.set_minimized(false);
                    settings_window.set_focus();
                }
                UserEvent::MinimizeSettings => {
                    settings_window.set_minimized(true);
                }
                UserEvent::CloseSettings => {
                    settings_window.set_visible(false);
                }
                UserEvent::CloseRadial => {
                    radial_window.set_visible(false);
                }
                UserEvent::ReloadRunningApps => {
                    let apps = get_running_apps();
                    if let Ok(json_str) = serde_json::to_string(&apps) {
                        let js = format!("window.onRunningAppsLoaded({});", json_str);
                        let _ = settings_webview.evaluate_script(&js);
                    }
                }
                UserEvent::AppFileSelected(app_json) => {
                    let js = format!("window.onAppFileSelected({});", app_json);
                    let _ = settings_webview.evaluate_script(&js);
                }
                UserEvent::Quit => {
                    *control_flow = ControlFlow::Exit;
                }
            },
            Event::WindowEvent { window_id, event, .. } => {
                if window_id == radial_window_id {
                    match event {
                        WindowEvent::Focused(focused) => {
                            if !focused {
                                radial_window.set_visible(false);
                            }
                        }
                        WindowEvent::CloseRequested => {
                            radial_window.set_visible(false);
                        }
                        _ => {}
                    }
                } else if window_id == settings_window_id {
                    match event {
                        WindowEvent::CloseRequested => {
                            settings_window.set_visible(false);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    });
}
