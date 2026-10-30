use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
use windows::core::PWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningApp {
    pub name: String,
    pub executable: String,
    pub title: String,
}

pub fn get_running_apps() -> Vec<RunningApp> {
    struct EnumData {
        apps: Vec<RunningApp>,
        seen_exes: HashSet<String>,
    }

    let mut data = EnumData {
        apps: Vec::new(),
        seen_exes: HashSet::new(),
    };

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let data = &mut *(lparam.0 as *mut EnumData);

        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }

        let mut title_buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title_buf);
        if len == 0 {
            return BOOL(1);
        }

        let title = String::from_utf16_lossy(&title_buf[..len as usize]);
        let trimmed_title = title.trim();
        if trimmed_title.is_empty() || trimmed_title == "Default IME" || trimmed_title == "MSCTFIME UI" {
            return BOOL(1);
        }

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return BOOL(1);
        }

        if let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            let mut path_buf = [0u16; 1024];
            let mut size = path_buf.len() as u32;

            if QueryFullProcessImageNameW(handle, PROCESS_NAME_FORMAT(0), PWSTR(path_buf.as_mut_ptr()), &mut size).is_ok() {
                let full_path = String::from_utf16_lossy(&path_buf[..size as usize]);
                let exe_name = Path::new(&full_path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("")
                    .to_string();

                let exe_lower = exe_name.to_lowercase();
                if !exe_name.is_empty()
                    && !exe_lower.contains("explorer.exe")
                    && !exe_lower.contains("shellexperiencehost.exe")
                    && !exe_lower.contains("searchhost.exe")
                    && !exe_lower.contains("halo.exe")
                    && !data.seen_exes.contains(&exe_lower)
                {
                    data.seen_exes.insert(exe_lower);
                    let display_name = Path::new(&exe_name)
                        .file_stem()
                        .and_then(|f| f.to_str())
                        .unwrap_or(&exe_name)
                        .to_string();

                    data.apps.push(RunningApp {
                        name: display_name,
                        executable: exe_name,
                        title: trimmed_title.to_string(),
                    });
                }
            }
            let _ = CloseHandle(handle);
        }

        BOOL(1)
    }

    unsafe {
        let lparam = LPARAM(&mut data as *mut EnumData as isize);
        let _ = EnumWindows(Some(enum_windows_proc), lparam);
    }

    data.apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    data.apps
}

pub fn get_active_process_name() -> String {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return String::new();
        }

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return String::new();
        }

        if let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            let mut path_buf = [0u16; 1024];
            let mut size = path_buf.len() as u32;

            if QueryFullProcessImageNameW(handle, PROCESS_NAME_FORMAT(0), PWSTR(path_buf.as_mut_ptr()), &mut size).is_ok() {
                let full_path = String::from_utf16_lossy(&path_buf[..size as usize]);
                let _ = CloseHandle(handle);
                if let Some(file_name) = Path::new(&full_path).file_name().and_then(|f| f.to_str()) {
                    return file_name.to_string();
                }
            } else {
                let _ = CloseHandle(handle);
            }
        }
    }
    String::new()
}
