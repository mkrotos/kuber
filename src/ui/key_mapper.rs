use crossterm::event::{KeyEvent, KeyCode};

use crate::event::Event;

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

pub enum InputAction {
    Quit,
    NextPod,
    PreviousPod,
}
