import sys
import os
import math
import json
import time

os.environ["QT_ENABLE_HIGHDPI_SCALING"] = "1"

from PySide6.QtWidgets import QApplication, QWidget, QSystemTrayIcon, QMenu, QStyle
from PySide6.QtCore import Qt, QPointF, QRectF, QThread, Signal, QVariantAnimation, QEasingCurve
from PySide6.QtGui import QPainter, QColor, QBrush, QPen, QFont, QRadialGradient, QCursor, QIcon, QAction, QPainterPath, QPixmap
from pynput import mouse
import pyautogui

# Import settings window & helper SVG
from settings_window import SettingsWindow, render_recolored_svg

# Helper Function agar path selalu valid di Dev & PyInstaller EXE
def get_resource_path(relative_path):
    if hasattr(sys, '_MEIPASS'):
        return os.path.join(sys._MEIPASS, relative_path)
    return os.path.join(os.path.abspath("."), relative_path)

CONFIG_PATH = get_resource_path("config.json")

class MouseListenerThread(QThread):
    trigger_signal = Signal(int, int)

    def __init__(self):
        super().__init__()
        self.last_click_time = 0
        self.DOUBLE_CLICK_GAP = 0.35

    def run(self):
        def on_click(x, y, button, pressed):
            if button == mouse.Button.middle and pressed:
                current_time = time.time()
                if (current_time - self.last_click_time) <= self.DOUBLE_CLICK_GAP:
                    self.trigger_signal.emit(x, y)
                    self.last_click_time = 0
                else:
                    self.last_click_time = current_time

        with mouse.Listener(on_click=on_click) as listener:
            listener.join()

class ShortcutRing(QWidget):
    open_settings_signal = Signal()

    def __init__(self):
        super().__init__()
        self.setWindowFlags(
            Qt.WindowType.FramelessWindowHint | 
            Qt.WindowType.WindowStaysOnTopHint | 
            Qt.WindowType.Tool
        )
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)
        self.setMouseTracking(True)
        self.resize(520, 520)

        self.center_point = QPointF(260, 260)
        self.hovered_index = -1
        self.hover_center = False
        
        self.load_config()

        self.anim_progress = 0.0
        self.scale_animation = QVariantAnimation(self)
        self.scale_animation.setStartValue(0.0)
        self.scale_animation.setEndValue(1.0)
        self.scale_animation.setDuration(220)
        self.scale_animation.setEasingCurve(QEasingCurve.Type.OutBack)
        self.scale_animation.valueChanged.connect(self._on_anim_step)

    def load_config(self):
        path = get_resource_path("config.json")
        if os.path.exists(path):
            with open(path, "r") as f:
                self.config = json.load(f)
        else:
            self.config = {"theme": "light", "base_radius": 120, "actions": []}

        self.actions = self.config.get("actions", [])
        self.base_radius = self.config.get("base_radius", 120)
        self.theme = self.config.get("theme", "light")

    def _on_anim_step(self, value):
        self.anim_progress = value
        self.update()

    def show_at(self, x, y):
        self.load_config()  # Memastikan config selalu ter-refresh sebelum muncul
        self.move(x - self.width() // 2, y - self.height() // 2)
        self.hovered_index = -1
        self.hover_center = False
        self.show()
        self.raise_()
        self.activateWindow()
        
        self.scale_animation.stop()
        self.scale_animation.start()

    def execute_action(self, action_item):
        act_type = action_item.get("type", "hotkey")
        val = action_item.get("value", "")

        if act_type == "internal" and val == "open_settings":
            self.open_settings_signal.emit()
            return

        if act_type == "hotkey":
            keys = [k.strip() for k in val.split("+")]
            if len(keys) == 1:
                pyautogui.press(keys[0])
            else:
                pyautogui.hotkey(*keys)

    def mouseMoveEvent(self, event):
        cursor_pos = event.position()
        total_items = len(self.actions)
        
        dist_center = math.hypot(cursor_pos.x() - self.center_point.x(), cursor_pos.y() - self.center_point.y())
        new_hover = -1
        new_center_hover = False

        if dist_center <= 18:
            new_center_hover = True
        else:
            current_radius = self.base_radius * self.anim_progress
            for i in range(total_items):
                angle_deg = (i * (360 / total_items)) - 90
                angle_rad = math.radians(angle_deg)

                bx = self.center_point.x() + (current_radius * math.cos(angle_rad))
                by = self.center_point.y() + (current_radius * math.sin(angle_rad))

                dist = math.hypot(cursor_pos.x() - bx, cursor_pos.y() - by)
                if dist <= 28:
                    new_hover = i
                    break

        if new_hover != self.hovered_index or new_center_hover != self.hover_center:
            self.hovered_index = new_hover
            self.hover_center = new_center_hover
            if self.hovered_index != -1 or self.hover_center:
                self.setCursor(QCursor(Qt.CursorShape.PointingHandCursor))
            else:
                self.setCursor(QCursor(Qt.CursorShape.ArrowCursor))
            self.update()

    def mousePressEvent(self, event):
        if event.button() == Qt.MouseButton.LeftButton:
            click_pos = event.position()
            total_items = len(self.actions)
            current_radius = self.base_radius * self.anim_progress

            for i, act in enumerate(self.actions):
                angle_deg = (i * (360 / total_items)) - 90
                angle_rad = math.radians(angle_deg)

                bx = self.center_point.x() + (current_radius * math.cos(angle_rad))
                by = self.center_point.y() + (current_radius * math.sin(angle_rad))

                dist = math.hypot(click_pos.x() - bx, click_pos.y() - by)
                if dist <= 28:
                    self.hide()
                    self.execute_action(act)
                    return

            self.hide()

    def paintEvent(self, event):
        if self.anim_progress <= 0:
            return

        painter = QPainter(self)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)

        total_items = len(self.actions)
        current_radius = self.base_radius * self.anim_progress
        opacity = min(1.0, max(0.0, self.anim_progress))

        painter.setOpacity(opacity)
        is_dark = self.theme == "dark"

        bg_btn = QColor("#0D0D0D") if is_dark else QColor("#ffffff")
        border_btn = QColor("#2D2D2D") if is_dark else QColor("#EDEDED")
        svg_color = QColor("#FFFFFF") if is_dark else QColor("#4A4A4A")

        for i, act in enumerate(self.actions):
            angle_deg = (i * (360 / total_items)) - 90
            angle_rad = math.radians(angle_deg)

            bx = self.center_point.x() + (current_radius * math.cos(angle_rad))
            by = self.center_point.y() + (current_radius * math.sin(angle_rad))
            btn_pos = QPointF(bx, by)

            is_hovered = (i == self.hovered_index)
            is_settings_btn = act.get("value") == "open_settings"

            # Node Button
            painter.setBrush(QBrush(bg_btn))
            painter.setPen(QPen(QColor("#1060FF") if is_hovered else border_btn, 3.0 if is_hovered else 1.5))
            painter.drawEllipse(btn_pos, 22, 22)

            # Draw Icons dengan get_resource_path
            if is_settings_btn:
                icon_path = get_resource_path(os.path.join("assets", "app_icon.png"))
                if os.path.exists(icon_path):
                    pixmap = QPixmap(icon_path).scaled(22, 22, Qt.KeepAspectRatio, Qt.SmoothTransformation)
                    painter.drawPixmap(int(bx - 11), int(by - 11), pixmap)
            else:
                icon_file = act.get("icon", "")
                icon_path = get_resource_path(os.path.join("assets", "icons", icon_file))
                colored_icon = render_recolored_svg(icon_path, QColor("#1060FF") if is_hovered else svg_color, 20)
                if colored_icon:
                    painter.drawImage(QRectF(bx - 10, by - 10, 20, 20), colored_icon)

            # Label Box Tampil Hanya Saat HOVER
            if is_hovered:
                label_text = act.get("label", "")
                painter.setFont(QFont("Segoe UI", 9, QFont.Weight.Bold))
                
                fm = painter.fontMetrics()
                text_width = fm.horizontalAdvance(label_text)
                box_w = max(70, text_width + 18)
                box_h = 24

                cos_val = math.cos(angle_rad)
                sin_val = math.sin(angle_rad)

                if abs(cos_val) < 0.2:
                    box_x = bx - (box_w / 2)
                    box_y = by - 52 if sin_val < 0 else by + 32
                elif abs(sin_val) < 0.2:
                    box_x = bx - box_w - 32 if cos_val < 0 else bx + 32
                    box_y = by - (box_h / 2)
                else:
                    box_x = bx - box_w - 22 if cos_val < 0 else bx + 22
                    box_y = by - box_h - 14 if sin_val < 0 else by + 14

                rect_path = QPainterPath()
                rect_path.addRoundedRect(QRectF(box_x, box_y, box_w, box_h), 6, 6)

                painter.setBrush(QBrush(QColor("#1060FF")))
                painter.setPen(Qt.PenStyle.NoPen)
                painter.drawPath(rect_path)

                painter.setPen(QColor("#FFFFFF"))
                painter.drawText(QRectF(box_x, box_y, box_w, box_h), Qt.AlignmentFlag.AlignCenter, label_text)

        # Center Close Indicator
        center_bg = QColor("#252525") if is_dark else QColor("#EBEBEB")
        center_fg = QColor("#AAAAAA") if is_dark else QColor("#777777")
        painter.setBrush(QBrush(center_bg))
        painter.setPen(Qt.PenStyle.NoPen)
        painter.drawEllipse(self.center_point, 16, 16)
        painter.setPen(center_fg)
        painter.setFont(QFont("Segoe UI", 10, QFont.Weight.Bold))
        painter.drawText(
            QRectF(self.center_point.x() - 10, self.center_point.y() - 10, 20, 20),
            Qt.AlignmentFlag.AlignCenter,
            "✕"
        )

class NativeApp:
    def __init__(self):
        self.app = QApplication(sys.argv)
        self.app.setQuitOnLastWindowClosed(False)

        self.ring_menu = ShortcutRing()
        self.settings_win = None

        self.ring_menu.open_settings_signal.connect(self.open_settings)
        self.setup_tray()

        self.mouse_thread = MouseListenerThread()
        self.mouse_thread.trigger_signal.connect(self.ring_menu.show_at)
        self.mouse_thread.start()

    def setup_tray(self):
        self.tray = QSystemTrayIcon(self.app)
        icon_path = get_resource_path(os.path.join("assets", "app_icon.png"))
        
        if os.path.exists(icon_path):
            self.tray.setIcon(QIcon(icon_path))
        else:
            self.tray.setIcon(self.app.style().standardIcon(QStyle.StandardPixmap.SP_ComputerIcon))

        self.tray.setToolTip("OrbitalMouse")

        tray_menu = QMenu()
        action_settings = tray_menu.addAction("Settings")
        action_settings.triggered.connect(self.open_settings)

        action_quit = tray_menu.addAction("Exit")
        action_quit.triggered.connect(self.app.quit)

        self.tray.setContextMenu(tray_menu)
        self.tray.show()

    def open_settings(self):
        if not self.settings_win:
            self.settings_win = SettingsWindow(reload_callback=self.reload_config)
        self.settings_win.show()
        self.settings_win.raise_()

    def reload_config(self):
        self.ring_menu.load_config()

    def run(self):
        sys.exit(self.app.exec())

if __name__ == "__main__":
    native_app = NativeApp()
    native_app.run()