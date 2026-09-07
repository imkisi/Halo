import subprocess
import sys
import os

import PySide6
pyside_dir = os.path.dirname(PySide6.__file__)
qsvg_plugin = os.path.join(pyside_dir, "plugins", "imageformats")

cmd = [
    sys.executable, "-m", "PyInstaller",
    "--noconsole",
    "--onefile",
    "--clean",
    f"--icon={os.path.join('assets', 'app_icon.ico')}" if os.path.exists("assets/app_icon.ico") else "",
    "--add-data", f"assets{os.path.pathsep}assets",
    "--add-data", f"config.json{os.path.pathsep}.",
    "--add-data", f"{qsvg_plugin}{os.path.pathsep}PySide6/plugins/imageformats",
    "--hidden-import", "PySide6.QtSvg",
    "--hidden-import", "PySide6.QtSvgWidgets",
    "app.py"
]

cmd = [c for c in cmd if c]

print("Building Standalone Halo.exe...")
subprocess.run(cmd)
print("\nHalo.exe is ready in 'dist/'")