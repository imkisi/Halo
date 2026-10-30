use std::thread;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn parse_vk(key: &str) -> Option<VIRTUAL_KEY> {
    let k = key.trim().to_lowercase();
    match k.as_str() {
        "ctrl" | "control" => Some(VK_CONTROL),
        "shift" => Some(VK_SHIFT),
        "alt" => Some(VK_MENU),
        "win" | "cmd" | "super" | "windows" => Some(VK_LWIN),
        "playpause" => Some(VK_MEDIA_PLAY_PAUSE),
        "volumemute" | "mute" | "silent" => Some(VK_VOLUME_MUTE),
        "volumeup" => Some(VK_VOLUME_UP),
        "volumedown" => Some(VK_VOLUME_DOWN),
        "nexttrack" | "next" => Some(VK_MEDIA_NEXT_TRACK),
        "prevtrack" | "previous" => Some(VK_MEDIA_PREV_TRACK),
        "tab" => Some(VK_TAB),
        "space" => Some(VK_SPACE),
        "enter" | "return" => Some(VK_RETURN),
        "esc" | "escape" => Some(VK_ESCAPE),
        "backspace" => Some(VK_BACK),
        "del" | "delete" => Some(VK_DELETE),
        "left" => Some(VK_LEFT),
        "right" => Some(VK_RIGHT),
        "up" => Some(VK_UP),
        "down" => Some(VK_DOWN),
        "." | "period" => Some(VK_OEM_PERIOD),
        "," | "comma" => Some(VK_OEM_COMMA),
        "=" | "equal" => Some(VK_OEM_PLUS),
        "-" | "minus" => Some(VK_OEM_MINUS),
        "/" | "slash" => Some(VK_OEM_2),
        "\\" | "backslash" => Some(VK_OEM_5),
        ";" | "semicolon" => Some(VK_OEM_1),
        "'" | "quote" => Some(VK_OEM_7),
        "[" | "bracketleft" => Some(VK_OEM_4),
        "]" | "bracketright" => Some(VK_OEM_6),
        "`" | "grave" => Some(VK_OEM_3),
        "f1" => Some(VK_F1),
        "f2" => Some(VK_F2),
        "f3" => Some(VK_F3),
        "f4" => Some(VK_F4),
        "f5" => Some(VK_F5),
        "f6" => Some(VK_F6),
        "f7" => Some(VK_F7),
        "f8" => Some(VK_F8),
        "f9" => Some(VK_F9),
        "f10" => Some(VK_F10),
        "f11" => Some(VK_F11),
        "f12" => Some(VK_F12),
        s if s.len() == 1 => {
            let ch = s.chars().next().unwrap();
            if ch.is_ascii_alphabetic() {
                Some(VIRTUAL_KEY(ch.to_ascii_uppercase() as u16))
            } else if ch.is_ascii_digit() {
                Some(VIRTUAL_KEY(ch as u16))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn send_hotkey(hotkey_str: &str) {
    let parts: Vec<&str> = hotkey_str.split('+').map(|s| s.trim()).collect();
    let mut vks = Vec::new();

    for p in parts {
        if let Some(vk) = parse_vk(p) {
            vks.push(vk);
        }
    }

    if vks.is_empty() {
        return;
    }

    unsafe {
        // Press all keys down
        for &vk in &vks {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: vk,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0), // 0 = keydown
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }

        thread::sleep(Duration::from_millis(20));

        // Release all keys in reverse order
        for &vk in vks.iter().rev() {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: vk,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
    }
}
