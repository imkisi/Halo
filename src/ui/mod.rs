pub const APP_ICON_BASE64: &str = include_str!("../../assets/app_icon.base64");

#[allow(dead_code)]
pub fn get_icon_svg(icon_name: &str) -> &'static str {
    match icon_name {
        "play.svg" | "play" => include_str!("../../assets/icons/play.svg"),
        "note.svg" | "note" => include_str!("../../assets/icons/note.svg"),
        "taskview.svg" | "taskview" => include_str!("../../assets/icons/taskview.svg"),
        "lock.svg" | "lock" => include_str!("../../assets/icons/lock.svg"),
        "screenshot.svg" | "screenshot" => include_str!("../../assets/icons/screenshot.svg"),
        "emoji.svg" | "emoji" => include_str!("../../assets/icons/emoji.svg"),
        "folder.svg" | "folder" => include_str!("../../assets/icons/folder.svg"),
        "desktop.svg" | "desktop" => include_str!("../../assets/icons/desktop.svg"),
        "clipboard.svg" | "clipboard" => include_str!("../../assets/icons/clipboard.svg"),
        "paste.svg" | "paste" => include_str!("../../assets/icons/paste.svg"),
        "taskmanager.svg" | "taskmanager" => include_str!("../../assets/icons/taskmanager.svg"),
        "mute.svg" | "mute" | "silent.svg" | "silent" => include_str!("../../assets/icons/mute.svg"),
        "previous.svg" | "previous" => include_str!("../../assets/icons/previous.svg"),
        "next.svg" | "next" => include_str!("../../assets/icons/next.svg"),
        "volumeup.svg" | "volumeup" => include_str!("../../assets/icons/volumeup.svg"),
        "volumedown.svg" | "volumedown" => include_str!("../../assets/icons/volumedown.svg"),
        "left.svg" | "left" => include_str!("../../assets/icons/left.svg"),
        "right.svg" | "right" => include_str!("../../assets/icons/right.svg"),
        "copilot.svg" | "copilot" => include_str!("../../assets/icons/copilot.svg"),
        _ => "",
    }
}

pub fn get_radial_html() -> &'static str {
    include_str!("radial.html")
}

pub fn get_settings_html() -> &'static str {
    include_str!("settings.html")
}
