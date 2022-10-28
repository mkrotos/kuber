use std::{
    sync::mpsc::{self, Receiver, RecvError},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event as CEvent, KeyEvent},
    terminal::enable_raw_mode,
};

pub enum Event<T> {
    Input(T),
    Tick,
}

pub struct EventLoop {
    rx: Receiver<Event<KeyEvent>>,
}

impl EventLoop {
    pub fn start(tick_rate: Duration) -> EventLoop {
        enable_raw_mode().expect("can run in raw mode");
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || keep_emitting_events(tick_rate, tx));
        EventLoop { rx }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, RecvError> {
        self.rx.recv()
    }
}

fn keep_emitting_events(tick_rate: Duration, tx: mpsc::Sender<Event<KeyEvent>>) {
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
}
