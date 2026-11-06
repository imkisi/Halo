use eframe::egui::{
    self, Align2, Color32, FontId, Pos2, Rect, Rounding, Stroke, Vec2,
};
use std::f32::consts::PI;

use crate::config::{ActionItem, AppProfile};

pub enum RadialResult {
    None,
    Execute(ActionItem),
    Close,
}

pub fn render_radial_wheel(
    ui: &mut egui::Ui,
    profile: &AppProfile,
    theme: &str,
    base_radius: f32,
) -> RadialResult {
    let mut result = RadialResult::None;

    let rect = ui.max_rect();
    let center = rect.center();
    let painter = ui.painter();

    let is_dark = theme == "dark";
    let center_bg = if is_dark { Color32::from_rgb(37, 37, 37) } else { Color32::from_rgb(235, 235, 235) };
    let center_fg = if is_dark { Color32::from_rgb(170, 170, 170) } else { Color32::from_rgb(119, 119, 119) };
    let slot_bg = if is_dark { Color32::from_rgb(22, 22, 22) } else { Color32::from_rgb(255, 255, 255) };
    let slot_border = if is_dark { Color32::from_rgb(45, 45, 45) } else { Color32::from_rgb(237, 237, 237) };
    let slot_fg = if is_dark { Color32::from_rgb(255, 255, 255) } else { Color32::from_rgb(74, 74, 74) };
    let accent_blue = Color32::from_rgb(16, 96, 255);

    let mouse_pos = ui.input(|i| i.pointer.hover_pos());
    let clicked = ui.input(|i| i.pointer.primary_clicked());

    // 1. Center Close Node (radius 16)
    let center_radius = 16.0;
    let is_center_hovered = mouse_pos.map_or(false, |p| p.distance(center) <= center_radius);

    let center_draw_bg = if is_center_hovered {
        if is_dark { Color32::from_rgb(51, 51, 51) } else { Color32::from_rgb(223, 223, 223) }
    } else {
        center_bg
    };

    painter.circle_filled(center, center_radius, center_draw_bg);
    painter.text(
        center,
        Align2::CENTER_CENTER,
        "✕",
        FontId::proportional(13.0),
        center_fg,
    );

    if is_center_hovered && clicked {
        return RadialResult::Close;
    }

    // 2. Action Slots
    let actions = &profile.actions;
    let total = actions.len();
    let slot_radius = 24.0;

    let mut hovered_slot: Option<(usize, Pos2, f32)> = None;

    for (idx, act) in actions.iter().enumerate() {
        let angle_deg = (idx as f32 * (360.0 / total as f32)) - 90.0;
        let angle_rad = angle_deg * (PI / 180.0);
        let slot_pos = Pos2::new(
            center.x + base_radius * angle_rad.cos(),
            center.y + base_radius * angle_rad.sin(),
        );

        let is_hovered = mouse_pos.map_or(false, |p| p.distance(slot_pos) <= slot_radius);

        if is_hovered {
            hovered_slot = Some((idx, slot_pos, angle_rad));
        }

        let bg_color = slot_bg;
        let stroke_color = if is_hovered { accent_blue } else { slot_border };
        let stroke_width = if is_hovered { 3.0 } else { 2.0 };
        let fg_color = if is_hovered { accent_blue } else { slot_fg };

        // Outer Slot Circle
        painter.circle_filled(slot_pos, slot_radius, bg_color);
        painter.circle_stroke(slot_pos, slot_radius, Stroke::new(stroke_width, stroke_color));

        // Slot Icon / Text
        let is_settings = act.value == "open_settings";
        let display_glyph = if is_settings {
            "⚙"
        } else {
            match act.icon.as_str() {
                "play.svg" | "play" => "▶",
                "note.svg" | "note" => "📝",
                "taskview.svg" | "taskview" => "🗔",
                "lock.svg" | "lock" => "🔒",
                "screenshot.svg" | "screenshot" => "✂",
                "emoji.svg" | "emoji" => "☺",
                "folder.svg" | "folder" => "📁",
                "desktop.svg" | "desktop" => "🖥",
                "clipboard.svg" | "clipboard" => "📋",
                "paste.svg" | "paste" => "📄",
                "mute.svg" | "mute" | "silent.svg" | "silent" => "🔇",
                "previous.svg" | "previous" => "⏮",
                "next.svg" | "next" => "⏭",
                "volumeup.svg" | "volumeup" => "🔊",
                "volumedown.svg" | "volumedown" => "🔉",
                _ => "●",
            }
        };

        painter.text(
            slot_pos,
            Align2::CENTER_CENTER,
            display_glyph,
            FontId::proportional(16.0),
            fg_color,
        );

        if is_hovered && clicked {
            result = RadialResult::Execute(act.clone());
        }
    }

    // 3. Render Tooltip Pill for Hovered Slot
    if let Some((idx, slot_pos, angle_rad)) = hovered_slot {
        let act = &actions[idx];
        let pill_text = &act.label;

        // Position pill outside the ring
        let offset = 40.0;
        let pill_pos = Pos2::new(
            slot_pos.x + offset * angle_rad.cos(),
            slot_pos.y + offset * angle_rad.sin(),
        );

        let font_id = FontId::proportional(12.0);
        let galley = ui.painter().layout_no_wrap(pill_text.clone(), font_id.clone(), Color32::WHITE);
        let pill_size = Vec2::new(galley.size().x + 20.0, galley.size().y + 10.0);
        let pill_rect = Rect::from_center_size(pill_pos, pill_size);

        painter.rect_filled(pill_rect, Rounding::same(6.0), accent_blue);
        painter.text(
            pill_rect.center(),
            Align2::CENTER_CENTER,
            pill_text,
            font_id,
            Color32::WHITE,
        );
    }

    // If clicked empty transparent background, close
    if clicked && matches!(result, RadialResult::None) {
        let mut clicked_any = is_center_hovered;
        for act_idx in 0..total {
            let angle_deg = (act_idx as f32 * (360.0 / total as f32)) - 90.0;
            let angle_rad = angle_deg * (PI / 180.0);
            let slot_pos = Pos2::new(
                center.x + base_radius * angle_rad.cos(),
                center.y + base_radius * angle_rad.sin(),
            );
            if mouse_pos.map_or(false, |p| p.distance(slot_pos) <= slot_radius) {
                clicked_any = true;
                break;
            }
        }
        if !clicked_any {
            result = RadialResult::Close;
        }
    }

    result
}

