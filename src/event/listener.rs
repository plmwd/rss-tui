use std::{sync::mpsc, thread, time::Duration};

use crossterm::event;

use super::keys::Key;

#[derive(Debug)]
pub struct EventListener {
    rx: mpsc::Receiver<Event<Key>>,
    tx: mpsc::Sender<Event<Key>>,
}

#[derive(Debug)]
pub enum Event<I> {
    Input(I),
    Tick,
}

impl EventListener {
    pub fn new(tick_rate: u64) -> Self {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(tick_rate);

        // TODO: Why is a clone needed?
        let listener_tx = tx.clone();
        thread::spawn(move || loop {
            if event::poll(tick_rate).unwrap() {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    let key = Key::from(key_event);
                    listener_tx.send(Event::Input(key)).unwrap();
                }
            }

            listener_tx.send(Event::Tick).unwrap();
        });

        EventListener { rx, tx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
