use macroquad::prelude::*;

use super::{Command, MenuLayout, bevel, theme};
use crate::logic::Difficulty;

/// Draws the menu bar and, if `open`, the dropdown on top of everything else.
/// `current` gets the check mark.
pub fn draw_menu(layout: &MenuLayout, open: bool, current: Difficulty, font: &Font) {
    // The bar, with a thin line separating it from the window below.
    let bar = layout.bar;
    draw_rectangle(bar.x, bar.y, bar.w, bar.h, theme::BACKGROUND);
    draw_rectangle(bar.x, bar.bottom() - 1.0, bar.w, 1.0, theme::BEVEL_DARK);

    // The title: highlighted while its menu is open, like in Windows.
    let title_color = if open {
        let t = layout.title;
        draw_rectangle(t.x, t.y, t.w, t.h, theme::MENU_HIGHLIGHT);
        theme::MENU_HIGHLIGHT_TEXT
    } else {
        theme::MENU_TEXT
    };
    draw_label(
        "Game",
        layout.title.x + theme::MENU_PADDING,
        layout.title,
        title_color,
        font,
    );

    if !open {
        return;
    }

    bevel::draw_raised(layout.dropdown, theme::BUTTON_BEVEL, theme::BACKGROUND);
    let mouse = Vec2::from(mouse_position());

    for item in &layout.items {
        let rect = item.rect;
        let hovered = rect.contains(mouse);
        let color = if hovered {
            // Inset so the highlight doesn't cover the dropdown's raised edge.
            let b = theme::BUTTON_BEVEL;
            draw_rectangle(rect.x + b, rect.y, rect.w - 2.0 * b, rect.h, theme::MENU_HIGHLIGHT);
            theme::MENU_HIGHLIGHT_TEXT
        } else {
            theme::MENU_TEXT
        };

        if item.command == Command::Start(current) {
            draw_check_mark(rect, color);
        }
        // Labels leave room on the left for the check mark.
        draw_label(item.label, rect.x + theme::MENU_CHECK_WIDTH, rect, color, font);

        let shortcut_width = measure_text(item.shortcut, Some(font), theme::MENU_FONT_SIZE, 1.0).width;
        let shortcut_x = rect.right() - theme::MENU_PADDING - shortcut_width;
        draw_label(item.shortcut, shortcut_x, rect, color, font);
    }
}

/// Text starting at `x`, vertically centred in `row`.
fn draw_label(text: &str, x: f32, row: Rect, color: Color, font: &Font) {
    let size = measure_text(text, Some(font), theme::MENU_FONT_SIZE, 1.0);
    let y = row.y + (row.h - size.height) / 2.0 + size.offset_y;
    let params = TextParams {
        font: Some(font),
        font_size: theme::MENU_FONT_SIZE,
        color,
        ..Default::default()
    };
    draw_text_ex(text, x, y, params);
}

/// A ✓ made of two lines, in the left margin of `row`.
/// (Drawn instead of typed: Roboto has no check-mark character.)
fn draw_check_mark(row: Rect, color: Color) {
    let x = row.x + theme::MENU_PADDING;
    let y = row.center().y;
    draw_line(x, y, x + 4.0, y + 4.0, 2.0, color);
    draw_line(x + 4.0, y + 4.0, x + 10.0, y - 4.0, 2.0, color);
}
