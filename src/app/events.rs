use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

use crate::app::Config;
use crate::Result;

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    Exit,
    NextTab,
    PreviousTab,
    Tick,
}

#[derive(Debug)]
pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn from_config(config: &Config) -> EventHandler {
        let (tx, rx) = mpsc::channel();

        // Thread than will handle user input and send events to receiver
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                trace!("Input thread spawned");
                for possible_key in stdin.keys() {
                    if let Ok(key) = possible_key {
                        let event = match key {
                            Key::Left => Event::PreviousTab,
                            Key::Right => Event::NextTab,
                            Key::Char('q') => Event::Exit,
                            Key::Ctrl('c') => Event::Exit,
                            Key::Esc => Event::Exit,
                            _ => continue,
                        };
                        let is_exit = event == Event::Exit;

                        if let Err(e) = tx.send(event) {
                            // Now that's just terrible thing to do with poor thread :(
                            warn!("Input thread failed to send event and will be terminated: {:?}", e);
                            return;
                        }

                        // User had requested an exit, closing this thread too
                        if is_exit {
                            trace!("Input thread just sent the Exit event and going to terminate now");
                            return;
                        }
                    }
                }
            })
        };

        // Thread that will "tick" with some user-defined interval.
        // Application might update state and re-draw UI on that event
        let interval = *config.delay();
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let tx = tx.clone();
                trace!("Tick thread is spawned with {:?} interval", interval);
                loop {
                    tx.send(Event::Tick).expect("Tick receiver is dead");
                    thread::sleep(interval);
                }
            })
        };

        EventHandler {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event> {
        match self.rx.recv() {
            Ok(event) => {
                trace!("UI thread had received an event: {:?}", event);
                Ok(event)
            }
            Err(e) => Err(e.into()),
        }
    }
}
