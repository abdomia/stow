use crossterm::event::{KeyEvent, MouseEvent};

pub enum EventState {
    Key(KeyEvent),
    Mouse(MouseEvent),
}
