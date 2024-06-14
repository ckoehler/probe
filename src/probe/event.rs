use tokio_stream::StreamExt;

use crossterm::event::KeyEvent;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::error;

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wraps keyboard input and tick events. Each event
/// type is handled in its own task and returned to a common `Receiver`
#[derive(Debug)]
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
        let (tx, rx) = mpsc::channel(10);
        {
            let tx = tx.clone();
            let mut reader = crossterm::event::EventStream::new();
            tokio::spawn(async move {
                loop {
                    if let Ok(crossterm::event::Event::Key(key)) = reader.next().await.unwrap() {
                        if let Err(err) = tx.send(Event::Input(key)).await {
                            error!("{}", err);
                        }
                    }
                }
            });
        }
        {
            let tx = tx.clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(config.tick_rate);
                loop {
                    if tx.send(Event::Tick).await.is_err() {
                        break;
                    }
                    interval.tick().await;
                }
            })
        };
        Events { rx }
    }

    pub async fn next(&mut self) -> Option<Event<KeyEvent>> {
        self.rx.recv().await
    }
}
