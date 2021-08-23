use crossterm::event::{read, Event, KeyCode, KeyEvent};

pub fn get_key_code() -> Option<KeyCode> {
    match read().unwrap() {
        Event::Key(event) => match match_event(event) {
            Some(key_code) => Some(key_code),
            None => None,
        },
        _ => None,
    }
}

fn match_event(event: KeyEvent) -> Option<KeyCode> {
    let KeyEvent { code, .. } = event;
    match code {
        //* edit key */
        KeyCode::Char('e') => Some(code),
        KeyCode::Char('s') => Some(code),

        //* quit key */
        KeyCode::Esc => Some(code),

        //* confirm key */
        KeyCode::Char(' ') => Some(code),

        //* *** */
        KeyCode::Down => Some(code),
        KeyCode::Left => Some(code),
        KeyCode::Right => Some(code),
        KeyCode::Up => Some(code),

        _ => None,
    }
}
