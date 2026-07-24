use crate::app::AppFlow;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum Action {
    None,
    Quit,
    Send,
}

/// Handle a key press. Returns an [`Action`] the caller should react to.
pub fn handle_key(app: &mut AppFlow, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Action::Quit,

        // for vim motion movement scrolling
        KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => app.scroll_chat(-3),
        KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => app.scroll_chat(3),

        KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.prompt_area.delete_word()
        }
        KeyCode::Left if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.prompt_area.move_word_left();
        }
        KeyCode::Right if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.prompt_area.move_word_right();
        }
        KeyCode::Up => {
            app.prompt_area.move_line_up();
        }
        KeyCode::Down => {
            app.prompt_area.move_line_down();
        }
        KeyCode::PageUp => app.scroll_chat(-10),
        KeyCode::PageDown => app.scroll_chat(10),
        KeyCode::Enter if key.modifiers.is_empty() => {
            app.send_message();
            return Action::Send;
        }
        KeyCode::Enter if key.modifiers.contains(KeyModifiers::ALT) => {
            app.prompt_area.insert_newline();
        }
        _ => app.prompt_area.input(key),
    }

    Action::None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Message;
    use crate::app::Role;
    use crossterm::event::{KeyCode, KeyEvent};

    #[test]
    fn ctrl_j_scrolls_down() {
        let mut flow = AppFlow::init_flow();
        flow.messages.push(Message {
            role: Role::User,
            content: "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8".into(),
        });

        flow.chat_scroll = 4;
        handle_key(
            &mut flow,
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL),
        );
        assert_eq!(flow.chat_scroll, 7);
    }

    #[test]
    fn ctrl_k_scrolls_up() {
        let mut flow = AppFlow::init_flow();
        flow.messages.push(Message {
            role: Role::User,
            content: "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8".into(),
        });
        flow.chat_scroll = 10;
        handle_key(
            &mut flow,
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL),
        );
        assert_eq!(flow.chat_scroll, 7);
    }
}
