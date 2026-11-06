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

    let is_dark = theme != "light";

    // Palette inspired by Logitech Actions Ring / OpenLogi
    let accent_cyan = Color32::from_rgb(0, 195, 255);
    let accent_blue = Color32::from_rgb(16, 120, 255);

    let (bg_slot, border_slot, fg_slot, center_bg, center_border, center_fg) = if is_dark {
        (
            Color32::from_rgba_unmultiplied(26, 28, 34, 235),
            Color32::from_rgba_unmultiplied(255, 255, 255, 30),
            Color32::from_rgb(240, 242, 245),
            Color32::from_rgba_unmultiplied(20, 22, 26, 240),
            Color32::from_rgba_unmultiplied(255, 255, 255, 40),
            Color32::from_rgb(220, 225, 230),
        )
    } else {
        (
            Color32::from_rgba_unmultiplied(255, 255, 255, 245),
            Color32::from_rgba_unmultiplied(0, 0, 0, 35),
            Color32::from_rgb(30, 32, 38),
            Color32::from_rgba_unmultiplied(245, 247, 250, 250),
            Color32::from_rgba_unmultiplied(0, 0, 0, 40),
            Color32::from_rgb(50, 55, 65),
        )
    };

    let mouse_pos = ui.input(|i| i.pointer.hover_pos());
    let clicked = ui.input(|i| i.pointer.primary_clicked());

    // 1. Draw subtle guide ring connecting the orbit (OpenLogi style)
    let guide_stroke = Stroke::new(
        1.5_f32,
        if is_dark {
            Color32::from_rgba_unmultiplied(255, 255, 255, 25)
        } else {
            Color32::from_rgba_unmultiplied(0, 0, 0, 20)
        },
    );
    painter.circle_stroke(center, base_radius, guide_stroke);

    // 2. Center Hub (Shows App Profile Name or Close Glyph)
    let center_radius = 32.0_f32;
    let is_center_hovered = mouse_pos.map_or(false, |p| p.distance(center) <= center_radius);

    let center_fill = if is_center_hovered {
        if is_dark {
            Color32::from_rgb(45, 48, 56)
        } else {
            Color32::from_rgb(230, 235, 240)
        }
    } else {
        center_bg
    };

    painter.circle_filled(center, center_radius, center_fill);
    painter.circle_stroke(
        center,
        center_radius,
        Stroke::new(if is_center_hovered { 2.5_f32 } else { 1.5_f32 }, if is_center_hovered { accent_cyan } else { center_border }),
    );

    if is_center_hovered {
        painter.text(
            center,
            Align2::CENTER_CENTER,
            "✕",
            FontId::proportional(16.0),
            if is_dark { Color32::WHITE } else { Color32::BLACK },
        );
    } else {
        // App Name in center
        let short_name = if profile.name.len() > 8 {
            format!("{}..", &profile.name[..7])
        } else {
            profile.name.clone()
        };
        painter.text(
            center,
            Align2::CENTER_CENTER,
            short_name,
            FontId::proportional(11.0),
            center_fg,
        );
    }

    if is_center_hovered && clicked {
        return RadialResult::Close;
    }

    // 3. Action Slots
    let actions = &profile.actions;
    let total = actions.len();
    let default_slot_radius = 26.0_f32;

    let mut hovered_slot: Option<(usize, Pos2, f32)> = None;

    for (idx, act) in actions.iter().enumerate() {
        let angle_deg = (idx as f32 * (360.0_f32 / total as f32)) - 90.0_f32;
        let angle_rad = angle_deg * (PI / 180.0_f32);
        let slot_pos = Pos2::new(
            center.x + base_radius * angle_rad.cos(),
            center.y + base_radius * angle_rad.sin(),
        );

        let is_hovered = mouse_pos.map_or(false, |p| p.distance(slot_pos) <= (default_slot_radius + 6.0_f32));

        if is_hovered {
            hovered_slot = Some((idx, slot_pos, angle_rad));
        }

        let slot_r = if is_hovered { 30.0_f32 } else { default_slot_radius };
        let stroke_color = if is_hovered { accent_cyan } else { border_slot };
        let stroke_width = if is_hovered { 3.0_f32 } else { 1.5_f32 };
        let icon_color = if is_hovered { accent_cyan } else { fg_slot };

        // Subtle drop glow when hovered
        if is_hovered {
            painter.circle_filled(
                slot_pos,
                slot_r + 4.0_f32,
                Color32::from_rgba_unmultiplied(0, 195, 255, 50),
            );
        }

        // Slot Pad
        painter.circle_filled(slot_pos, slot_r, bg_slot);
        painter.circle_stroke(slot_pos, slot_r, Stroke::new(stroke_width, stroke_color));

        // Slot Icon Glyph
        let glyph = action_glyph(&act.icon, &act.value);
        painter.text(
            slot_pos,
            Align2::CENTER_CENTER,
            glyph,
            FontId::proportional(if is_hovered { 18.0 } else { 15.0 }),
            icon_color,
        );

        if is_hovered && clicked {
            result = RadialResult::Execute(act.clone());
        }
    }

    // 4. Floating Action Badge / Tooltip for Hovered Slot
    if let Some((idx, slot_pos, angle_rad)) = hovered_slot {
        let act = &actions[idx];
        let label_text = &act.label;
        let hotkey_hint = if act.r#type == "hotkey" && !act.value.is_empty() {
            format!("  [{}]", act.value.to_uppercase())
        } else {
            String::new()
        };

        let badge_text = format!("{}{}", label_text, hotkey_hint);

        // Position pill radially outward from the slot
        let pill_dist = 46.0_f32;
        let pill_pos = Pos2::new(
            slot_pos.x + pill_dist * angle_rad.cos(),
            slot_pos.y + pill_dist * angle_rad.sin(),
        );

        let font_id = FontId::proportional(12.0);
        let galley = ui.painter().layout_no_wrap(badge_text.clone(), font_id.clone(), Color32::WHITE);
        let pill_size = Vec2::new(galley.size().x + 22.0, galley.size().y + 12.0);
        let pill_rect = Rect::from_center_size(pill_pos, pill_size);

        // Draw pill background with rounded corners & accent border
        painter.rect_filled(
            pill_rect,
            Rounding::same(8.0),
            if is_dark { Color32::from_rgb(18, 20, 24) } else { Color32::from_rgb(30, 32, 38) },
        );
        painter.rect_stroke(
            pill_rect,
            Rounding::same(8.0),
            Stroke::new(1.5_f32, accent_blue),
        );
        painter.text(
            pill_rect.center(),
            Align2::CENTER_CENTER,
            badge_text,
            font_id,
            Color32::WHITE,
        );
    }

    // Dismiss if clicked empty space
    if clicked && matches!(result, RadialResult::None) {
        let mut clicked_any = is_center_hovered;
        for act_idx in 0..total {
            let angle_deg = (act_idx as f32 * (360.0_f32 / total as f32)) - 90.0_f32;
            let angle_rad = angle_deg * (PI / 180.0_f32);
            let slot_pos = Pos2::new(
                center.x + base_radius * angle_rad.cos(),
                center.y + base_radius * angle_rad.sin(),
            );
            if mouse_pos.map_or(false, |p| p.distance(slot_pos) <= (default_slot_radius + 8.0_f32)) {
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

fn action_glyph(icon: &str, value: &str) -> &'static str {
    if value == "open_settings" {
        return "⚙";
    }
    match icon {
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
}
