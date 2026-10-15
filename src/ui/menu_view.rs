use macroquad::prelude::*;

use super::{Command, MenuLayout, bevel, theme};
use crate::logic::Difficulty;
use crate::scores::BestTimes;

/// Draws the menu bar and, if a menu is `open` (index into `layout.menus`), its dropdown on
/// top of everything else. `current` gets the check mark; each difficulty shows its best time.
pub fn draw_menu(layout: &MenuLayout, open: Option<usize>, current: Difficulty, best: &BestTimes, font: &Font) {
    // The bar, with a thin line separating it from the window below.
    let bar = layout.bar;
    draw_rectangle(bar.x, bar.y, bar.w, bar.h, theme::BACKGROUND);
    draw_rectangle(bar.x, bar.bottom() - 1.0, bar.w, 1.0, theme::BEVEL_DARK);

    // The titles: the open one highlighted, like in Windows.
    for (index, menu) in layout.menus.iter().enumerate() {
        let t = menu.title_rect;
        let color = if open == Some(index) {
            draw_rectangle(t.x, t.y, t.w, t.h, theme::MENU_HIGHLIGHT);
            theme::MENU_HIGHLIGHT_TEXT
        } else {
            theme::MENU_TEXT
        };
        draw_label(menu.title, t.x + theme::MENU_PADDING, t, color, font);
    }

    let Some(menu) = open.map(|index| &layout.menus[index]) else {
        return;
    };

    bevel::draw_raised(menu.dropdown, theme::BUTTON_BEVEL, theme::BACKGROUND);
    let mouse = Vec2::from(mouse_position());

    for item in &menu.items {
        let rect = item.rect;
        let hovered = rect.contains(mouse);
        let (color, dim_color) = if hovered {
            // Inset so the highlight doesn't cover the dropdown's raised edge.
            let b = theme::BUTTON_BEVEL;
            draw_rectangle(rect.x + b, rect.y, rect.w - 2.0 * b, rect.h, theme::MENU_HIGHLIGHT);
            (theme::MENU_HIGHLIGHT_TEXT, theme::MENU_HIGHLIGHT_TEXT)
        } else {
            (theme::MENU_TEXT, theme::MENU_DIM_TEXT)
        };

        if item.command == Command::Start(current) {
            draw_check_mark(rect, color);
        }
        // Labels leave room on the left for the check mark.
        draw_label(item.label, rect.x + theme::MENU_CHECK_WIDTH, rect, color, font);

        // Best time, right-aligned just left of the shortcut column. Nothing until it's been won.
        if let Command::Start(difficulty) = item.command
            && let Some(seconds) = best.get(difficulty)
        {
            let text = format!("{seconds} s");
            let right = rect.right() - theme::MENU_PADDING - theme::MENU_SHORTCUT_WIDTH;
            draw_label_right_aligned(&text, right, rect, dim_color, font);
        }

        draw_label_right_aligned(item.shortcut, rect.right() - theme::MENU_PADDING, rect, color, font);
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

/// Text ending at `right`, vertically centred in `row`.
fn draw_label_right_aligned(text: &str, right: f32, row: Rect, color: Color, font: &Font) {
    let width = measure_text(text, Some(font), theme::MENU_FONT_SIZE, 1.0).width;
    draw_label(text, right - width, row, color, font);
}

/// A ✓ made of two lines, in the left margin of `row`.
/// (Drawn instead of typed: Roboto has no check-mark character.)
fn draw_check_mark(row: Rect, color: Color) {
    let x = row.x + theme::MENU_PADDING;
    let y = row.center().y;
    draw_line(x, y, x + 4.0, y + 4.0, 2.0, color);
    draw_line(x + 4.0, y + 4.0, x + 10.0, y - 4.0, 2.0, color);
}
