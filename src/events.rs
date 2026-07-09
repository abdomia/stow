use crossterm::event::KeyEvent;

use crate::tui::TextArea;

pub fn update_input(text_input: &mut TextArea, keys: KeyEvent) {
    text_input.input(keys);
}
