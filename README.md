# Halo

A high-performance, standalone radial shortcut menu for Windows with per-application profiles.

---

## Overview

Halo is a lightweight desktop utility designed to streamline repetitive interactions and window management on Windows 10 and 11. Built entirely in Rust, Halo runs silently in the system tray and summons an interactive radial menu directly beneath the mouse cursor upon a double-click of the middle mouse button (scroll wheel).

Unlike browser-backed or runtime-dependent utilities, Halo is compiled as a native, single-file binary with zero external dependencies and a minimal memory footprint (under 25 MB RAM).

---

## Features

- **Instant Mouse Invocation**: Activates at the current cursor coordinates via a low-level Win32 mouse hook when the middle mouse button is double-clicked.
- **Per-Application Context Profiles**: Automatically detects the active foreground application (such as Adobe Photoshop, After Effects, or custom executables) and loads its dedicated shortcut wheel.
- **Native Vector Interface**: Renders transparent, hardware-accelerated UI elements using `egui` and OpenGL (`glow`) with per-pixel alpha transparency and no window frame artifacts.
- **Zero Browser Dependencies**: Operates without Chromium or WebView2 runtimes, preventing unnecessary background helper processes and privacy prompt triggers.
- **Configurable Shortcuts**: Assign system actions, media controls, global hotkeys, or custom multi-key combinations with variable radial wheel sizing (80px to 180px) and dark/light themes.
- **System Tray Integration**: Provides a persistent system tray menu to toggle overlays, open the configuration editor, or exit cleanly.

---

## System Requirements

- **Operating System**: Windows 10 or Windows 11 (64-bit)
- **Architecture**: x86_64

---

## Installation

### Pre-built Executable

1. Download the latest `Halo-Windows-x64.zip` package from the **Releases** or GitHub Actions artifacts.
2. Extract the archive to any desired directory.
3. Launch `Halo.exe`. The application will initialize silently in the Windows system tray.

### Building from Source

Ensure the Rust toolchain (stable MSVC target) is installed.

```powershell
# Clone the repository
git clone https://github.com/your-username/Halo.git
cd Halo

# Verify configuration and compile in release mode
cargo build --release --target x86_64-pc-windows-msvc
```

The compiled executable will be located at:
`target/x86_64-pc-windows-msvc/release/halo.exe`

---

## Usage

- **Summon Radial Menu**: Rapidly double-click the Middle Mouse Button (scroll wheel) anywhere on screen.
- **Select Action**: Click an action slot to trigger the assigned hotkey. The radial menu will close automatically.
- **Dismiss Menu**: Press `Escape`, click the center close node (`✕`), or click anywhere outside the radial wheel.
- **Open Settings**: Right-click the system tray icon and select **Halo Settings**, or trigger the internal settings shortcut from the default radial menu.

---

## Configuration

Settings are persisted in `config.json` adjacent to the executable. If no configuration file is detected on launch, a default profile is generated automatically.

### Structure Example

```json
{
  "theme": "dark",
  "base_radius": 120,
  "active_app_id": "default",
  "apps": [
    {
      "id": "default",
      "name": "Default",
      "executable": "*",
      "icon": "default",
      "actions": [
        {
          "label": "Play/Pause",
          "icon": "play.svg",
          "type": "hotkey",
          "value": "playpause",
          "category": "MEDIA & VOLUME"
        },
        {
          "label": "Screenshot",
          "icon": "screenshot.svg",
          "type": "hotkey",
          "value": "win+shift+s",
          "category": "NAVIGATION"
        },
        {
          "label": "Halo Settings",
          "icon": "app_icon.png",
          "type": "internal",
          "value": "open_settings",
          "category": "SYSTEM"
        }
      ]
    }
  ],
  "custom_shortcuts": [
    {
      "label": "Cmd+Shift+=",
      "value": "ctrl+shift+=",
      "category": "CUSTOM"
    }
  ]
}
```

---

## Architecture

- **`src/main.rs`**: Application lifecycle, multi-viewport event coordination, and system tray management.
- **`src/hook.rs`**: Global low-level mouse hook (`WH_MOUSE_LL`) tracking double-tap timing and cursor positions.
- **`src/process.rs`**: Active foreground window detection and running process enumeration via Win32 APIs.
- **`src/input.rs`**: Low-level synthetic keyboard input dispatch using `SendInput`.
- **`src/config.rs`**: Profile serialization, schema definitions, and file persistence.
- **`src/ui/radial.rs`**: Vector rendering of the transparent radial overlay, slot math, and hover states.
- **`src/ui/settings.rs`**: Profile management interface, action palette, and key capture dialogs.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
