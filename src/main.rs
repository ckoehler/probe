mod probe;

use crate::probe::app::App;
use crate::probe::config::{Cli, Probes};
#[allow(dead_code)]
use crate::probe::event::{Config, Event, Events};
use crate::probe::inputs::Inputs;
use crate::probe::ui;

use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{error::Error, io, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    // get config
    let cli: Cli = argh::from_env();
    let config = fs::read_to_string(cli.config).expect("Something went wrong reading the file");
    let probes: Probes = toml::from_str(&config).unwrap();
    // println!("{:?}", probes);

    // set up terminal
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // setup inputs
    let inputs = Inputs::with_probes(probes.probes.clone());

    // set up events and app
    let events = Events::with_config_and_probes(Config {
        tick_rate: Duration::from_millis(cli.tick_rate),
        ..Config::default()
    });
    let app = App::new("Probe", probes.probes);
    let app = Arc::new(Mutex::new(app));

    // input loop
    let tapp = Arc::clone(&app);
    thread::spawn(move || loop {
        match inputs.next() {
            msg => {
                let msg = msg.unwrap();
                let mut app = tapp.lock().unwrap();
                app.process_message_for_stream(msg.0, msg.1);
            }
        }
    });

    // event loop
    loop {
        {
            let app = app.lock().unwrap();
            terminal.draw(|f| ui::draw(f, &app))?;
        }

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => {
                    let mut app = app.lock().unwrap();
                    app.on_key(c);
                }
                Key::Up => {
                    let mut app = app.lock().unwrap();
                    app.on_up();
                }
                Key::Down => {
                    let mut app = app.lock().unwrap();
                    app.on_down();
                }
                Key::Left => {
                    let mut app = app.lock().unwrap();
                    app.on_left();
                }
                Key::Right => {
                    let mut app = app.lock().unwrap();
                    app.on_right();
                }
                _ => {}
            },
            Event::Tick => {
                let mut app = app.lock().unwrap();
                app.on_tick();
            }
        }
        let app = app.lock().unwrap();
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
