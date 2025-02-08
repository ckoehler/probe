use crossterm::event::read;
use crossterm::event::KeyEvent;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tracing::info;

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        {
            let tx = tx.clone();
            thread::spawn(move || loop {
                if let Ok(crossterm::event::Event::Key(key)) = read() {
                    if let Err(err) = tx.send(Event::Input(key)) {
                        eprintln!("{}", err);
                    }
                }
            });
        }
        {
            let tx = tx.clone();
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                info!("Tick");
                thread::sleep(config.tick_rate);
            })
        };
        Events { rx }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}
