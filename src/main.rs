mod probe;
#[allow(dead_code)]
mod util;

use crate::probe::{ui, App};
use crate::util::event::{Config, Event, Events};
use argh::FromArgs;
use serde::Deserialize;
use std::fs;
use std::{error::Error, io, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

/// Probe Config
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
    /// config file.
    #[argh(option, default = "String::from(\"prober.toml\")")]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Probes {
    probes: Vec<Probe>,
}
#[derive(Debug, Deserialize)]
struct Probe {
    address: String,
    mode: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let config = fs::read_to_string(cli.config).expect("Something went wrong reading the file");

    let deserialized: Probes = toml::from_str(&config).unwrap();
    println!("{:?}", deserialized);

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(cli.tick_rate),
        ..Config::default()
    });

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("Probe", cli.enhanced_graphics);
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => {
                    app.on_key(c);
                }
                Key::Up => {
                    app.on_up();
                }
                Key::Down => {
                    app.on_down();
                }
                Key::Left => {
                    app.on_left();
                }
                Key::Right => {
                    app.on_right();
                }
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
