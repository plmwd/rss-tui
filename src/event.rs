use std::{sync::mpsc, thread, time::Duration};

use crossterm::{
    self,
    event::{self, KeyEvent},
};
use tokio::time::Instant;

#[derive(Debug)]
pub struct EventListener {
    rx: mpsc::Receiver<Event<Key>>,
    _tx: mpsc::Sender<Event<Key>>,
}

#[derive(Debug)]
pub enum Event<I> {
    Input(I),
    Tick,
}

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

impl EventListener {
    pub fn new(tick_rate: u64) -> Self {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(tick_rate);

        let sender = tx.clone();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).unwrap() {
                    if let event::Event::Key(key_event) = event::read().unwrap() {
                        sender.send(Event::Input(Key::from(key_event))).unwrap();
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        EventListener { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
