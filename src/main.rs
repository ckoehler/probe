mod probe;

use crate::probe::app::App;
use crate::probe::config::{Cli, Probes};
use crate::probe::event::{Config, Event, Events};
use crate::probe::inputs::Inputs;
use crate::probe::state::AppState;
use crate::probe::ui;
use cli_log::*;
use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{error::Error, io, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // get logging macros
    init_cli_log!();
    // get config
    let cli: Cli = argh::from_env();
    let config = fs::read_to_string(cli.config).expect("Something went wrong reading the file");
    let probes: Probes = toml::from_str(&config).unwrap();
    probes.validate();
    // println!("{:?}", probes);

    // set up terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // setup inputs
    let inputs = Inputs::with_probes(probes.probes.clone());

    // set up events and app
    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(cli.tick_rate),
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
            Event::Input(key) => match key.code {
                KeyCode::Char(c) => {
                    let mut app = app.lock().unwrap();
                    app.on_key(c);
                }
                KeyCode::Enter => {
                    let mut app = app.lock().unwrap();
                    app.on_key('\n');
                }
                KeyCode::Up => {
                    let mut app = app.lock().unwrap();
                    app.on_up();
                }
                KeyCode::Down => {
                    let mut app = app.lock().unwrap();
                    app.on_down();
                }
                KeyCode::Left => {
                    let mut app = app.lock().unwrap();
                    app.on_left();
                }
                KeyCode::Right => {
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
            io::stdout().execute(LeaveAlternateScreen)?;
            disable_raw_mode()?;
            break;
        }
    }

    Ok(())
}
