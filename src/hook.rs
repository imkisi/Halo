use eframe::egui;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::process::get_active_process_name;

#[derive(Debug, Clone)]
pub enum UserEvent {
    ShowRadial {
        x: i32,
        y: i32,
        process_name: String,
    },
    OpenSettings,
    CloseRadial,
    Quit,
}

static LAST_CLICK_TIME: Mutex<Option<Instant>> = Mutex::new(None);
static EVENT_SENDER: Mutex<Option<(Sender<UserEvent>, egui::Context)>> = Mutex::new(None);
static HOOK_HANDLE: AtomicUsize = AtomicUsize::new(0);

pub fn start_mouse_hook(sender: Sender<UserEvent>, ctx: egui::Context) {
    {
        let mut s = EVENT_SENDER.lock().unwrap();
        *s = Some((sender, ctx));
    }

    thread::spawn(|| unsafe {
        let hook = SetWindowsHookExW(
            WH_MOUSE_LL,
            Some(mouse_ll_proc),
            None,
            0,
        );

        if let Ok(h) = hook {
            HOOK_HANDLE.store(h.0 as usize, Ordering::SeqCst);

            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let _ = UnhookWindowsHookEx(h);
        }
    });
}

unsafe extern "system" fn mouse_ll_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if ncode >= 0 && wparam.0 as u32 == WM_MBUTTONUP {
        let now = Instant::now();
        let mut last_click = LAST_CLICK_TIME.lock().unwrap();

        // Use Windows system double click threshold or default to 450ms
        let system_gap = GetDoubleClickTime();
        let gap_ms = if system_gap > 0 { system_gap as u128 } else { 450 };

        let is_double_click = if let Some(prev) = *last_click {
            now.duration_since(prev).as_millis() <= gap_ms
        } else {
            false
        };

        if is_double_click {
            *last_click = None;

            let mut pt = POINT::default();
            let _ = GetCursorPos(&mut pt);
            let active_proc = get_active_process_name();

            let sender_opt = EVENT_SENDER.lock().unwrap();
            if let Some((sender, ctx)) = sender_opt.as_ref() {
                let _ = sender.send(UserEvent::ShowRadial {
                    x: pt.x,
                    y: pt.y,
                    process_name: active_proc,
                });
                // Crucial for instantaneous responsiveness: wake up egui event loop immediately!
                ctx.request_repaint();
            }
        } else {
            *last_click = Some(now);
        }
    }

    CallNextHookEx(None, ncode, wparam, lparam)
}
