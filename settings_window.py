import sys
import os
import json
import math
from PySide6.QtWidgets import (
    QMainWindow, QWidget, QHBoxLayout, QVBoxLayout, 
    QLabel, QFrame, QPushButton,
    QListWidget, QListWidgetItem, QAbstractItemView, QSlider, QSystemTrayIcon, QStyle
)
from PySide6.QtCore import Qt, QPointF, QRectF, QMimeData, QPoint, QSize
from PySide6.QtGui import QPainter, QColor, QBrush, QPen, QFont, QPainterPath, QDrag, QPixmap, QIcon
from PySide6.QtSvg import QSvgRenderer

PRESET_ACTIONS = [
    {"label": "Emoji Panel", "type": "hotkey", "value": "win+.", "category": "KEYBOARD", "icon": "emoji.svg"},
    {"label": "Play/Pause", "type": "hotkey", "value": "playpause", "category": "MEDIA & VOLUME", "icon": "play.svg"},
    {"label": "New Note", "type": "hotkey", "value": "win+n", "category": "SYSTEM", "icon": "note.svg"},
    {"label": "Copilot", "type": "hotkey", "value": "win+c", "category": "ADVANCED", "icon": "copilot.svg"},
    {"label": "Lock Workstation", "type": "hotkey", "value": "win+l", "category": "SYSTEM", "icon": "lock.svg"},
    {"label": "Screenshot", "type": "hotkey", "value": "win+shift+s", "category": "NAVIGATION", "icon": "screenshot.svg"},
    {"label": "File Explorer", "type": "hotkey", "value": "win+e", "category": "OPEN", "icon": "folder.svg"},
    {"label": "Task View", "type": "hotkey", "value": "win+tab", "category": "NAVIGATION", "icon": "taskview.svg"},
    {"label": "Paste Plain Text", "type": "hotkey", "value": "ctrl+shift+v", "category": "KEYBOARD", "icon": "paste.svg"},
    {"label": "Clipboard History", "type": "hotkey", "value": "win+v", "category": "KEYBOARD", "icon": "clipboard.svg"},
    {"label": "Show Desktop", "type": "hotkey", "value": "win+d", "category": "SYSTEM", "icon": "desktop.svg"},
    {"label": "Mute Audio", "type": "hotkey", "value": "volumemute", "category": "MEDIA & VOLUME", "icon": "mute.svg"},
    {"label": "Task Manager", "type": "hotkey", "value": "ctrl+shift+esc", "category": "SYSTEM", "icon": "taskmanager.svg"},
    {"label": "Virtual Desktop Left", "type": "hotkey", "value": "win+ctrl+left", "category": "NAVIGATION", "icon": "left.svg"},
    {"label": "Virtual Desktop Right", "type": "hotkey", "value": "win+ctrl+right", "category": "NAVIGATION", "icon": "right.svg"}
]

def get_resource_path(relative_path):
    if hasattr(sys, '_MEIPASS'):
        return os.path.join(sys._MEIPASS, relative_path)
    return os.path.join(os.path.abspath("."), relative_path)

CONFIG_PATH = get_resource_path("config.json")

def setup_tray(self):
    self.tray = QSystemTrayIcon()
    icon_path = get_resource_path(os.path.join("assets", "app_icon.png"))
    if os.path.exists(icon_path):
        self.tray.setIcon(QIcon(icon_path))
    else:
        self.tray.setIcon(self.app.style().standardIcon(QStyle.StandardPixmap.SP_ComputerIcon))

def render_recolored_svg(svg_path, color, size):
    if not os.path.exists(svg_path):
        return None

    renderer = QSvgRenderer(svg_path)
    if not renderer.isValid():
        return None

    from PySide6.QtGui import QImage
    image = QImage(size, size, QImage.Format.Format_ARGB32)
    image.fill(Qt.GlobalColor.transparent)

    painter = QPainter(image)
    renderer.render(painter)
    
    painter.setCompositionMode(QPainter.CompositionMode.CompositionMode_SourceIn)
    painter.fillRect(image.rect(), color)
    painter.end()

    return image

class RingDropPreviewWidget(QWidget):
    def __init__(self, settings_window, actions_data, theme="light", base_radius=120):
        super().__init__()
        self.settings_window = settings_window
        self.actions = actions_data
        self.theme = theme
        self.base_radius = base_radius
        self.hover_slot_idx = -1
        self.setMinimumSize(480, 480)
        self.setAcceptDrops(True)

    def dragEnterEvent(self, event):
        if event.mimeData().hasText():
            event.acceptProposedAction()

    def dragMoveEvent(self, event):
        if event.mimeData().hasText():
            drop_pos = event.position()
            center = QPointF(self.width() / 2, self.height() / 2)
            total = len(self.actions)

            closest_idx = -1
            min_dist = float("inf")
            for i in range(total):
                angle_rad = math.radians((i * (360 / total)) - 90)
                bx = center.x() + (self.base_radius * math.cos(angle_rad))
                by = center.y() + (self.base_radius * math.sin(angle_rad))
                dist = math.hypot(drop_pos.x() - bx, drop_pos.y() - by)
                if dist < min_dist:
                    min_dist = dist
                    closest_idx = i

            if closest_idx != -1 and self.actions[closest_idx].get("value") == "open_settings":
                self.hover_slot_idx = -1
            else:
                self.hover_slot_idx = closest_idx

            self.update()
            event.acceptProposedAction()

    def dragLeaveEvent(self, event):
        self.hover_slot_idx = -1
        self.update()

    def dropEvent(self, event):
        action_json = event.mimeData().text()
        self.hover_slot_idx = -1
        try:
            new_action = json.loads(action_json)
            drop_pos = event.position()
            center = QPointF(self.width() / 2, self.height() / 2)
            total = len(self.actions)

            closest_idx = 0
            min_dist = float("inf")
            for i in range(total):
                angle_rad = math.radians((i * (360 / total)) - 90)
                bx = center.x() + (self.base_radius * math.cos(angle_rad))
                by = center.y() + (self.base_radius * math.sin(angle_rad))
                dist = math.hypot(drop_pos.x() - bx, drop_pos.y() - by)
                if dist < min_dist:
                    min_dist = dist
                    closest_idx = i

            if self.actions[closest_idx].get("value") == "open_settings":
                return

            self.actions[closest_idx] = new_action
            self.settings_window.save_config()
            self.update()
            event.acceptProposedAction()
        except Exception:
            pass

    def paintEvent(self, event):
        painter = QPainter(self)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)

        center = QPointF(self.width() / 2, self.height() / 2)
        total = len(self.actions)
        is_dark = self.theme == "dark"

        bg_btn = QColor("#0D0D0D") if is_dark else QColor("#ffffff")
        border_btn = QColor("#2D2D2D") if is_dark else QColor("#EDEDED")
        svg_color = QColor("#FFFFFF") if is_dark else QColor("#4A4A4A")

        for i, act in enumerate(self.actions):
            angle_deg = (i * (360 / total)) - 90
            angle_rad = math.radians(angle_deg)

            bx = center.x() + (self.base_radius * math.cos(angle_rad))
            by = center.y() + (self.base_radius * math.sin(angle_rad))
            btn_pos = QPointF(bx, by)

            is_settings_btn = act.get("value") == "open_settings"
            is_target_hover = (i == self.hover_slot_idx)

            if is_target_hover:
                painter.setBrush(QBrush(QColor("#1060FF")))
                painter.setPen(QPen(QColor("#1060FF"), 4))
                painter.drawEllipse(btn_pos, 26, 26)
            else:
                painter.setBrush(QBrush(bg_btn))
                painter.setPen(QPen(border_btn, 3))
                painter.drawEllipse(btn_pos, 22, 22)

            if is_settings_btn:
                icon_path = get_resource_path("assets/app_icon.png")
                if os.path.exists(icon_path):
                    pixmap = QPixmap(icon_path).scaled(22, 22, Qt.KeepAspectRatio, Qt.SmoothTransformation)
                    painter.setOpacity(0.75)
                    painter.drawPixmap(int(bx - 11), int(by - 11), pixmap)
                    painter.setOpacity(1.0)
            else:
                icon_file = act.get("icon", "")
                icon_path = get_resource_path(os.path.join("assets", "icons", icon_file))
                colored_icon = render_recolored_svg(icon_path, QColor("#FFFFFF") if is_target_hover else svg_color, 20)
                if colored_icon:
                    painter.drawImage(QRectF(bx - 10, by - 10, 20, 20), colored_icon)

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
                box_y = by - 60 if sin_val < 0 else by + 40
            elif abs(sin_val) < 0.2:
                box_x = bx - box_w - 40 if cos_val < 0 else bx + 40
                box_y = by - (box_h / 2)
            else:
                box_x = bx - box_w - 30 if cos_val < 0 else bx + 30
                box_y = by - box_h - 20 if sin_val < 0 else by + 20

            rect_path = QPainterPath()
            rect_path.addRoundedRect(QRectF(box_x, box_y, box_w, box_h), 6, 6)

            painter.setBrush(QBrush(QColor("#1060FF")))
            painter.setPen(Qt.PenStyle.NoPen)
            painter.drawPath(rect_path)

            painter.setPen(QColor("#FFFFFF"))
            painter.drawText(QRectF(box_x, box_y, box_w, box_h), Qt.AlignmentFlag.AlignCenter, label_text)

        center_bg = QColor("#252525") if is_dark else QColor("#EBEBEB")
        center_fg = QColor("#AAAAAA") if is_dark else QColor("#777777")
        painter.setBrush(QBrush(center_bg))
        painter.setPen(Qt.PenStyle.NoPen)
        painter.drawEllipse(center, 14, 14)
        painter.setPen(center_fg)
        painter.drawText(QRectF(center.x()-10, center.y()-10, 20, 20), Qt.AlignmentFlag.AlignCenter, "✕")

class DraggableActionList(QListWidget):
    def __init__(self):
        super().__init__()
        self.setDragEnabled(True)
        self.setSelectionMode(QAbstractItemView.SelectionMode.SingleSelection)
        self.setFocusPolicy(Qt.FocusPolicy.NoFocus)

    def startDrag(self, supportedActions):
        item = self.currentItem()
        if not item:
            return
        
        action_data = item.data(Qt.ItemDataRole.UserRole)
        mime_data = QMimeData()
        mime_data.setText(json.dumps(action_data))

        drag = QDrag(self)
        drag.setMimeData(mime_data)

        pixmap_w = 180
        pixmap_h = 42
        drag_pixmap = QPixmap(pixmap_w, pixmap_h)
        drag_pixmap.fill(Qt.GlobalColor.transparent)

        painter = QPainter(drag_pixmap)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)

        painter.setOpacity(0.85)
        rect_path = QPainterPath()
        rect_path.addRoundedRect(QRectF(2, 2, pixmap_w - 4, pixmap_h - 4), 20, 20)
        
        painter.setBrush(QBrush(QColor("#1060FF")))
        painter.setPen(QPen(QColor("#FFFFFF"), 2))
        painter.drawPath(rect_path)

        icon_file = action_data.get("icon", "")
        icon_path = get_resource_path(os.path.join("assets", "icons", icon_file))
        colored_icon = render_recolored_svg(icon_path, QColor("#FFFFFF"), 18)
        
        if colored_icon:
            painter.drawImage(QRectF(14, 12, 18, 18), colored_icon)

        painter.setPen(QColor("#FFFFFF"))
        painter.setFont(QFont("Segoe UI", 9, QFont.Weight.Bold))
        text_rect = QRectF(38, 0, pixmap_w - 48, pixmap_h)
        painter.drawText(text_rect, Qt.AlignmentFlag.AlignVCenter | Qt.AlignmentFlag.AlignLeft, action_data.get("label", ""))

        painter.end()

        drag.setPixmap(drag_pixmap)
        drag.setHotSpot(QPoint(pixmap_w // 2, pixmap_h // 2))
        drag.exec_(Qt.DropAction.CopyAction)

class SettingsWindow(QMainWindow):
    def __init__(self, reload_callback=None):
        super().__init__()
        self.reload_callback = reload_callback
        
        self.setWindowFlags(Qt.WindowType.FramelessWindowHint)
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)
        self.resize(980, 640)
        self.drag_position = QPoint()

        self.load_config()
        self.init_ui()
        self.apply_theme_style(self.config.get("theme", "light"))

    def load_config(self):
        if os.path.exists(CONFIG_PATH):
            with open(CONFIG_PATH, "r") as f:
                self.config = json.load(f)
        else:
            self.config = {"theme": "light", "base_radius": 120, "actions": []}

    def save_config(self):
        with open(CONFIG_PATH, "w") as f:
            json.dump(self.config, f, indent=2)
        if self.reload_callback:
            self.reload_callback()

    def populate_action_list(self):
        self.action_list.clear()
        is_dark = self.config.get("theme", "light") == "dark"
        svg_color = QColor("#FFFFFF") if is_dark else QColor("#4A4A4A")

        for act in PRESET_ACTIONS:
            item = QListWidgetItem(f"  {act['label']}")
            item.setData(Qt.ItemDataRole.UserRole, act)

            icon_file = act.get("icon", "")
            icon_path = get_resource_path(os.path.join("assets", "icons", icon_file))
            colored_icon = render_recolored_svg(icon_path, svg_color, 20)
            
            if colored_icon:
                item.setIcon(QIcon(QPixmap.fromImage(colored_icon)))

            self.action_list.addItem(item)

    def init_ui(self):
        self.container_frame = QFrame()
        self.setCentralWidget(self.container_frame)
        
        outer_layout = QVBoxLayout(self.container_frame)
        outer_layout.setContentsMargins(0, 0, 0, 0)
        outer_layout.setSpacing(0)

        self.title_bar = QWidget()
        self.title_bar.setFixedHeight(38)
        title_layout = QHBoxLayout(self.title_bar)
        title_layout.setContentsMargins(16, 0, 8, 0)

        self.lbl_title = QLabel("Halo (Preview)")
        self.lbl_title.setStyleSheet("font-weight: 700; font-size: 13px; background: transparent;")
        title_layout.addWidget(self.lbl_title)
        title_layout.addStretch()

        self.btn_min = QPushButton("─")
        self.btn_close = QPushButton("✕")

        for btn in (self.btn_min, self.btn_close):
            btn.setFixedSize(32, 28)
            btn.setFocusPolicy(Qt.FocusPolicy.NoFocus)
            title_layout.addWidget(btn)

        self.btn_min.clicked.connect(self.showMinimized)
        self.btn_close.clicked.connect(self.close)

        outer_layout.addWidget(self.title_bar)

        content_widget = QWidget()
        main_layout = QHBoxLayout(content_widget)
        main_layout.setContentsMargins(0, 0, 0, 0)

        left_panel = QWidget()
        left_layout = QVBoxLayout(left_panel)
        left_layout.setContentsMargins(20, 10, 20, 20)

        self.preview_ring = RingDropPreviewWidget(
            self, 
            self.config.get("actions", []), 
            self.config.get("theme", "light"),
            self.config.get("base_radius", 120)
        )
        left_layout.addWidget(self.preview_ring, alignment=Qt.AlignmentFlag.AlignCenter)
        main_layout.addWidget(left_panel, stretch=6)

        self.right_panel = QFrame()
        right_layout = QVBoxLayout(self.right_panel)
        right_layout.setContentsMargins(24, 20, 24, 20)

        theme_layout = QHBoxLayout()
        self.lbl_theme = QLabel("Theme")
        theme_layout.addWidget(self.lbl_theme)
        theme_layout.addStretch()

        self.btn_theme_light = QPushButton("Light")
        self.btn_theme_dark = QPushButton("Dark")
        
        self.btn_theme_light.setFixedWidth(60)
        self.btn_theme_dark.setFixedWidth(60)
        
        self.btn_theme_light.setFocusPolicy(Qt.FocusPolicy.NoFocus)
        self.btn_theme_dark.setFocusPolicy(Qt.FocusPolicy.NoFocus)

        self.btn_theme_light.clicked.connect(lambda: self.change_theme("light"))
        self.btn_theme_dark.clicked.connect(lambda: self.change_theme("dark"))

        segmented_box = QFrame()
        seg_layout = QHBoxLayout(segmented_box)
        seg_layout.setContentsMargins(2, 2, 2, 2)
        seg_layout.setSpacing(0)
        seg_layout.addWidget(self.btn_theme_light)
        seg_layout.addWidget(self.btn_theme_dark)

        theme_layout.addWidget(segmented_box)
        right_layout.addLayout(theme_layout)

        radius_layout = QVBoxLayout()
        radius_layout.setContentsMargins(0, 15, 0, 15)
        self.lbl_radius = QLabel("Halo Radius")
        radius_layout.addWidget(self.lbl_radius)

        self.slider_radius = QSlider(Qt.Orientation.Horizontal)
        self.slider_radius.setMinimum(80)
        self.slider_radius.setMaximum(180)
        self.slider_radius.setValue(self.config.get("base_radius", 120))
        self.slider_radius.setFocusPolicy(Qt.FocusPolicy.NoFocus)
        self.slider_radius.valueChanged.connect(self.change_radius)
        radius_layout.addWidget(self.slider_radius)
        right_layout.addLayout(radius_layout)

        self.lbl_available = QLabel("Actions")
        right_layout.addWidget(self.lbl_available)

        self.action_list = DraggableActionList()
        self.action_list.setIconSize(QSize(20, 20))
        self.populate_action_list()

        right_layout.addWidget(self.action_list)
        main_layout.addWidget(self.right_panel, stretch=4)

        outer_layout.addWidget(content_widget)

    def mousePressEvent(self, event):
        if event.button() == Qt.MouseButton.LeftButton:
            if event.position().y() <= self.title_bar.height():
                self.drag_position = event.globalPosition().toPoint() - self.frameGeometry().topLeft()
                event.accept()

    def mouseMoveEvent(self, event):
        if event.buttons() == Qt.MouseButton.LeftButton and not self.drag_position.isNull():
            self.move(event.globalPosition().toPoint() - self.drag_position)
            event.accept()

    def mouseReleaseEvent(self, event):
        self.drag_position = QPoint()

    def apply_theme_style(self, theme):
        is_dark = theme == "dark"

        if is_dark:
            bg_main = "#0D0D0D"
            text_color = "#ffffff"
            border_color = "#2D2D2D"
            track_bg = "#222222"
            scroll_handle = "#777777"
            btn_segmented_active = "#1060FF"
            btn_segmented_idle = "#1A1A1A"
        else:
            bg_main = "#f9f9f9"
            text_color = "#0D0D0D"
            border_color = "#EDEDED"
            track_bg = "#E0E0E0"
            scroll_handle = "#B0B0B0"
            btn_segmented_active = "#1060FF"
            btn_segmented_idle = "#EFEFEF"

        self.container_frame.setStyleSheet(f"""
            QFrame {{
                background-color: {bg_main};
                border: 1px solid {border_color};
                border-radius: 8px;
            }}
        """)

        self.setStyleSheet(f"QMainWindow, QWidget {{ background-color: {bg_main}; color: {text_color}; font-family: 'Segoe UI', sans-serif; }}")
        self.right_panel.setStyleSheet(f"QFrame {{ background-color: {bg_main}; border: none;}}")
        self.lbl_title.setStyleSheet("border: none; background: transparent; font-weight: 700; font-size: 13px;")
        self.lbl_theme.setStyleSheet("border: none; background: transparent; font-weight: 700; font-size: 14px;")
        self.lbl_radius.setStyleSheet("border: none; background: transparent; font-weight: 700; font-size: 14px;")
        self.lbl_available.setStyleSheet("border: none; background: transparent; font-weight: 700; font-size: 14px; margin-top: 10px;")

        active_light = f"background-color: {btn_segmented_active}; color: #FFFFFF;" if not is_dark else f"background-color: {btn_segmented_idle}; color: {text_color};"
        active_dark = f"background-color: {btn_segmented_active}; color: #FFFFFF;" if is_dark else f"background-color: {btn_segmented_idle}; color: {text_color};"

        self.btn_theme_light.setStyleSheet(f"QPushButton {{ border: none; border-radius: 6px; padding: 6px; font-weight: bold; {active_light} }}")
        self.btn_theme_dark.setStyleSheet(f"QPushButton {{ border: none; border-radius: 6px; padding: 6px; font-weight: bold; {active_dark} }}")

        btn_title_style = f"QPushButton {{ background: transparent; border: none; color: {text_color}; font-size: 12px; }} QPushButton:hover {{ background: {border_color}; }}"
        self.btn_min.setStyleSheet(btn_title_style)
        self.btn_close.setStyleSheet("QPushButton { background: transparent; border: none; color: #FF4D4D; font-size: 12px; } QPushButton:hover { background: #FF4D4D; color: #FFFFFF; }")

        self.slider_radius.setStyleSheet(f"""
            QSlider::groove:horizontal {{
                border: none;
                height: 14px;
                background: {track_bg};
                border-radius: 7px;
            }}
            QSlider::sub-page:horizontal {{
                background: #1060FF;
                border-radius: 7px;
            }}
            QSlider::handle:horizontal {{
                background: transparent;
                width: 0px;
                height: 0px;
            }}
        """)

        self.action_list.setStyleSheet(f"""
            QListWidget {{
                background-color: transparent;
                border: none;
                outline: none;
                padding-right: 0px;
            }}
            QListWidget::item {{
                padding: 0px 8px;
                margin-bottom: 6px;
                background-color: {bg_main};
                color: {text_color};
                border: 3px solid {border_color};
                border-radius: 20px;
                font-weight: 600;
            }}
            QListWidget::item:hover {{
                border-color: #1060FF;
                color: #1060FF;
            }}
            QScrollBar:vertical {{
                border: none;
                background: transparent;
                width: 4px;
                margin: 0px 0px 0px 10px;
            }}
            QScrollBar::handle:vertical {{
                background: {scroll_handle};
                min-height: 30px;
                border-radius: 2px;
            }}
            QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical {{
                height: 0px;
                background: transparent;
            }}
        """)

    def change_theme(self, new_theme):
        self.config["theme"] = new_theme
        self.save_config()
        self.apply_theme_style(new_theme)
        self.populate_action_list()
        self.preview_ring.theme = new_theme
        self.preview_ring.update()

    def change_radius(self, value):
        self.config["base_radius"] = value
        self.save_config()
        self.preview_ring.base_radius = value
        self.preview_ring.update()