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
static EVENT_SENDER: Mutex<Option<Sender<UserEvent>>> = Mutex::new(None);
static HOOK_HANDLE: AtomicUsize = AtomicUsize::new(0);

const DOUBLE_CLICK_GAP_MS: u128 = 350;

pub fn start_mouse_hook(sender: Sender<UserEvent>) {
    {
        let mut s = EVENT_SENDER.lock().unwrap();
        *s = Some(sender);
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

        let is_double_click = if let Some(prev) = *last_click {
            now.duration_since(prev).as_millis() <= DOUBLE_CLICK_GAP_MS
        } else {
            false
        };

        if is_double_click {
            *last_click = None;

            let mut pt = POINT::default();
            let _ = GetCursorPos(&mut pt);
            let active_proc = get_active_process_name();

            let sender_opt = EVENT_SENDER.lock().unwrap();
            if let Some(sender) = sender_opt.as_ref() {
                let _ = sender.send(UserEvent::ShowRadial {
                    x: pt.x,
                    y: pt.y,
                    process_name: active_proc,
                });
            }
        } else {
            *last_click = Some(now);
        }
    }

    CallNextHookEx(None, ncode, wparam, lparam)
}
