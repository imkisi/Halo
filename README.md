<div align="center">
  <img src="assets/app_icon.png" alt="Halo Logo" width="120" style="max-width: 100%; height: auto;" />
</div>

<h1 align="center">Halo - High Performance Radial Shortcut Menu for Windows</h1>

<div align="center">

![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-0078D6?style=for-the-badge&logo=windows)
![Python](https://img.shields.io/badge/Python-3.10%2B-3776AB?style=for-the-badge&logo=python)
![Framework](<https://img.shields.io/badge/UI-PySide6%20(Qt6)-41CD52?style=for-the-badge&logo=qt>)
![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)

**An open-source, sleek, and responsive radial shortcut menu inspired by the Logitech Actions Ring. It stays quietly in your system tray and summons instantly at your cursor with a double-click of the Middle Mouse Button.**

</div>

---

## 🌟 Highlights & Key Features

- **⚡ Instant Mouse Trigger**: Summon the radial menu directly beneath your mouse cursor anywhere in Windows with a rapid double-tap of the **Middle Mouse Button (Scroll Wheel)**.
- **⭕ Logitech Actions Ring Inspired**: Brings a modern, customizable radial workflow to any setup without requiring vendor-locked hardware or heavy software bloat.
- **🎨 Modern & Minimalist Design**:
  - Dark & Light mode toggle.
  - Smooth scale & bounce spring micro-animations.
  - Radial hover glows and dynamic floating tooltips.
  - Dynamic recoloring for vector SVG icons.
- **🚀 Background & System Tray Native**:
  - Runs silently in the background with zero terminal/console window popups.
  - System Tray icon for quick settings access and clean exit.
- **🎯 Drag & Drop Customization**:
  - Interactive visual ring configurator in Settings.
  - Drag and drop actions to reorder or replace slots on the fly.
  - Adjustable Halo radius slider (80px – 180px).
- **📦 Single Executable Standalone**:
  - Compiles into a completely self-contained `.exe` file with zero external directory dependencies.

---

## 📋 Table of Contents

1. [Installation](#-installation)
   - [Option A: Download Pre-built .exe (Releases)](#option-a-download-pre-built-exe-github-releases)
   - [Option B: Build from Source](#option-b-build-from-source)
2. [How It Works](#-how-it-works)
3. [Configuration & Custom Actions](#-configuration--custom-actions)
4. [Project Architecture](#-project-architecture)
5. [Troubleshooting & FAQ](#-troubleshooting--faq)

---

## 📥 Installation

### Option A: Download Pre-built `.exe` (GitHub Releases)

The easiest way to get started without installing Python:

1. Navigate to the **[GitHub Releases](../../releases)** section of this repository.
2. Download the latest `Halo.exe` standalone executable (or `Halo-Setup.exe` installer).
3. Double-click `Halo.exe` to run.
4. Check your **System Tray** (near the taskbar clock) to verify Halo is running, then **double-click the Middle Mouse Button** anywhere on screen!

---

### Option B: Build from Source

#### Prerequisites

- Windows 10 or 11 (64-bit)
- Python 3.10+ added to system `PATH`

#### 1. Clone Repository & Install Dependencies

````powershell
# Clone the repository
git clone [https://github.com/imkisi/Halo.git](https://github.com/imkisi/Halo.git)
cd Halo

# Install required dependencies
pip install -r requirements.txt

#### 2. Run From Source

```powershell
python app.py
````

- When started, Halo will display a brief tray notification confirming it is active.
- **Double-click the Middle Mouse Button** anywhere on your screen to summon the radial menu!

#### 3. Build Everything with One Click

To compile the application into a standalone .exe using PyInstaller Run:

```powershell
python build.py
```

This will automatically build:

- **`dist/Halo.exe`**: Standalone single-file portable application.
- **`dist/Halo-Setup.exe`**: Standalone graphical setup installer wizard.

---

## 🔍 How It Works

### Mouse Trigger & Global Hook

- **Global Listener**: Powered by a dedicated background thread running `pynput.mouse.Listener`.
- **Double-Click Timing**: Measures the interval between middle-click button releases (`DOUBLE_CLICK_GAP = 0.35s`).
- **Safety**: Safe stopping and cleanup hooks ensure no ghost hooks or unmanaged threads remain when exiting.

### System Tray & Frameless Overlay

- **Frameless Overlay**: The radial menu is a borderless, transparent `QWidget` configured with `Qt.WindowStaysOnTopHint | Qt.WindowType.Tool | Qt.WA_TranslucentBackground`.
- **System Tray**:
  - **Left-Click / Double-Click Tray Icon**: Opens Halo Settings.
  - **Right-Click Context Menu**: Access Settings or Exit application cleanly.

## ⚙️ Configuration & Custom Actions

Halo stores configuration in `config.json` placed alongside the application executable or source directory.

### Configuration Format (`config.json`)

```json
{
  "theme": "dark",
  "base_radius": 120,
  "actions": [
    {
      "label": "Screenshot",
      "icon": "screenshot.svg",
      "type": "hotkey",
      "value": "win+shift+s",
      "category": "NAVIGATION"
    },
    {
      "label": "Play/Pause",
      "icon": "play.svg",
      "type": "hotkey",
      "value": "playpause",
      "category": "MEDIA & VOLUME"
    },
    {
      "label": "Settings",
      "type": "internal",
      "value": "open_settings",
      "category": "SYSTEM"
    }
  ]
}
```

## 📂 Project Architecture

```
Halo/
├── main.py                # Core application entry, radial ring overlay & tray setup
├── settings_window.py     # Settings GUI, SVG renderer & drag-and-drop preview
├── build.py               # Automated PyInstaller build script
├── config.json            # Radial action slots configuration
├── requirements.txt       # Python package dependencies
└── assets/
    ├── app_icon.png       # Application icon PNG
    ├── app_icon.ico       # Windows executable icon
    └── icons/             # Action SVG icons
        ├── clipboard.svg
        ├── desktop.svg
        ├── emoji.svg
        ├── folder.svg
        ├── lock.svg
        └── ...
```

---

## 🛠️ Troubleshooting & FAQ

### Q: Halo doesn't appear when double-clicking the middle button.

1. Check the System Tray (notification area arrow near the clock) to ensure the Halo icon is visible and active.
2. If another application captures exclusive mouse hooks in administrator mode, run `Halo.exe` as Administrator so it can receive global mouse events.

### Q: How do I change the ring size?

Open **Settings** (click the app icon on the ring or tray icon) and adjust the **Halo Radius** slider.

---

<div align="center">
Made with ❤️ for high-productivity Windows workflows.
</div>
