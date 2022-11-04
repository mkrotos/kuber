use crossterm::event::{KeyCode, KeyEvent};

use self::event_loop::Event;

pub mod event_loop;

pub enum InputAction {
    Quit,
    NextPod,
    PreviousPod,
    FetchLogs,
    LogsUp,
    LogsDown,
    LogsEnd,
}

pub fn map_input(input: Event<KeyEvent>) -> Option<InputAction> {
    match input {
        Event::Input(event) => match event.code {
            KeyCode::Char('q') | KeyCode::F(12) => Some(InputAction::Quit),
            KeyCode::Down => Some(InputAction::NextPod),
            KeyCode::Up => Some(InputAction::PreviousPod),
            KeyCode::Enter => Some(InputAction::FetchLogs),
            KeyCode::PageUp => Some(InputAction::LogsUp),
            KeyCode::PageDown => Some(InputAction::LogsDown),
            KeyCode::End => Some(InputAction::LogsEnd),
            _ => None,
        },
        Event::Tick => None,
    }
}
