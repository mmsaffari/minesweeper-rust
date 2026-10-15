use macroquad::prelude::*;

use super::dialog_view::{draw_button, draw_dialog_frame, draw_text_at};
use super::{DialogButton, NumberDialog, NumberDialogLayout, bevel, theme};

/// Draws the "Choose game" dialog. `time` (from `get_time()`) blinks the text cursor.
pub fn draw_number_dialog(dialog: &NumberDialog, layout: &NumberDialogLayout, font: &Font, time: f64) {
    draw_dialog_frame(layout.panel, layout.title_bar, "Choose a game", font);

    let label = "Game number (1 to 999999):";
    draw_text_at(label, layout.label.x, layout.label, 13, theme::MENU_TEXT, font);

    // The field: white and sunken, like a classic text box. An invalid number shows in red.
    let field = layout.field;
    draw_rectangle(field.x, field.y, field.w, field.h, theme::FIELD_BACKGROUND);
    bevel::draw_sunken_border(field, theme::LED_BEVEL);
    let color = if dialog.number().is_some() {
        theme::MENU_TEXT
    } else {
        theme::EXPLODED
    };
    let text_x = field.x + theme::MENU_PADDING;
    draw_text_at(dialog.digits(), text_x, field, theme::NUMBER_FONT_SIZE, color, font);

    // A blinking cursor after the digits: on for half a second, off for half a second.
    if time.fract() < 0.5 {
        let width = measure_text(dialog.digits(), Some(font), theme::NUMBER_FONT_SIZE, 1.0).width;
        let x = text_x + width + 2.0;
        draw_line(x, field.y + 5.0, x, field.bottom() - 5.0, 1.5, theme::MENU_TEXT);
    }

    let pressed = dialog.pressed();
    draw_button(layout.ok, "OK", pressed == Some(DialogButton::Ok), font);
    draw_button(layout.cancel, "Cancel", pressed == Some(DialogButton::Cancel), font);
}
