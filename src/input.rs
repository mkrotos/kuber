use crossterm::event::{KeyCode, KeyEvent};

use self::event_loop::Event;

pub mod event_loop;

pub enum InputAction {
    Quit,
    NextPod,
    PreviousPod,
}

pub fn map_input(input: Event<KeyEvent>) -> Option<InputAction> {
    match input {
        Event::Input(event) => match event.code {
            KeyCode::Char('q') => Some(InputAction::Quit),
            KeyCode::Down => Some(InputAction::NextPod),
            KeyCode::Up => Some(InputAction::PreviousPod),
            _ => None,
        },
        Event::Tick => None,
    }
}
