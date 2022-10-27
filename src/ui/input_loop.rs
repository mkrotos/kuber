use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use crate::event::Event;
use crossterm::{
    event::{self, Event as CEvent, KeyEvent},
    terminal::enable_raw_mode,
};

pub fn start_input_loop() -> Result<Receiver<Event<KeyEvent>>, Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send event");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    Ok(rx)
}
