use crossterm::event::{self, KeyEvent};

pub enum Key {
    Char(char),
}

impl From<KeyEvent> for Key {
    fn from(key: KeyEvent) -> Self {
        if let event::KeyCode::Char(c) = key.code {
            Key::Char(c)
        } else {
            Key::Char('e')
        }
    }
}
