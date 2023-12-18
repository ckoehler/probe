mod probe;

use crate::probe::app::App;
use crate::probe::config::{Cli, Probes};
use crate::probe::event::{Config, Event, Events};
use crate::probe::inputs::Inputs;
use crate::probe::state::AppState;
use crate::probe::ui;

use ratatui::{backend::TermionBackend, Terminal};
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{error::Error, io, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::IntoAlternateScreen};

fn main() -> Result<(), Box<dyn Error>> {
    // get config
    let cli: Cli = argh::from_env();
    let config = fs::read_to_string(cli.config).expect("Something went wrong reading the file");
    let probes: Probes = toml::from_str(&config).unwrap();
    probes.validate();
    // println!("{:?}", probes);

    // set up terminal
    let stdout = io::stdout().into_raw_mode()?.into_alternate_screen()?;
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // setup inputs
    let inputs = Inputs::with_probes(probes.probes.clone());

    // set up events and app
    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(cli.tick_rate),
        ..Config::default()
    });
    let appstate = AppState::from_probes(probes.probes);
    let app = App::new("Probe", appstate);
    let app = Arc::new(Mutex::new(app));

    // input loop
    let tapp = Arc::clone(&app);
    thread::spawn(move || loop {
        let msg = inputs.next();
        {
            let msg = msg.unwrap();
            let mut app = tapp.lock().unwrap();
            app.process_message_for_stream(msg.0, msg.1);
        }
    });

    // event loop
    loop {
        {
            let mut app = app.lock().unwrap();
            terminal.draw(|f| ui::draw(f, &mut app))?;
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
