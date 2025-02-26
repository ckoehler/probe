use color_eyre::eyre::Result;
use tracing::info;
use tracing_error::ErrorLayer;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{self, Layer, layer::SubscriberExt};
mod probe;

use crate::probe::app::App;
use crate::probe::config::{Cli, Probes};
use crate::probe::event::{Config, Event, Events};
use crate::probe::inputs::Inputs;
use crate::probe::state::AppState;
use crate::probe::ui;
use crossterm::{
    ExecutableCommand,
    event::KeyCode,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::fs;
use std::sync::{Arc, Mutex};
use std::{error::Error, io, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // set up logging
    initialize_logging()?;

    // get config
    let cli: Cli = argh::from_env();
    let config = fs::read_to_string(cli.config).expect("Something went wrong reading the file");
    let probes: Probes = toml::from_str(&config).expect("Couldn't parse config file.");
    probes.validate();

    // set up terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // setup inputs
    let mut inputs = Inputs::with_probes(&probes.probes);

    // set up events and app
    let mut events = Events::with_config(Config {
        tick_rate: Duration::from_millis(cli.tick_rate),
    });
    let appstate = AppState::from_probes(&probes.probes);
    let app = App::new("Probe", appstate);
    let app = Arc::new(Mutex::new(app));

    // input loop
    let tapp = Arc::clone(&app);
    tokio::spawn(async move {
        loop {
            let msg = inputs.next().await.expect("Failed to get next input.");
            {
                let mut app = tapp.lock().expect("Failed to lock Mutex");
                app.process_message_for_stream(&msg.0, &msg.1);
            }
        }
    });
    // event loop
    loop {
        {
            let mut app = app.lock().expect("Failed to lock Mutex");
            terminal.draw(|f| ui::draw(f, &mut app))?;
        }

        match events.next().await {
            Some(Event::Input(key)) => match key.code {
                KeyCode::Char(c) => {
                    let mut app = app.lock().expect("Failed to lock Mutex");
                    app.on_key(c);
                }
                KeyCode::Enter => {
                    let mut app = app.lock().expect("Failed to lock Mutex");
                    app.on_key('\n');
                }
                KeyCode::Up => {
                    let mut app = app.lock().expect("Failed to lock Mutex");
                    app.on_up();
                }
                KeyCode::Down => {
                    let mut app = app.lock().expect("Failed to lock Mutex");
                    app.on_down();
                }
                KeyCode::Left => {
                    let mut app = app.lock().expect("Failed to lock Mutex");
                    app.on_left();
                }
                KeyCode::Right => {
                    let mut app = app.lock().expect("Failed to lock Mutex");
                    app.on_right();
                }
                _ => {}
            },
            Some(Event::Tick) => {
                info!("got tick");
                let mut app = app.lock().expect("Failed to lock Mutex");
                app.on_tick();
            }
            None => {}
        }

        let app = app.lock().expect("Failed to lock Mutex");
        if app.should_quit {
            io::stdout().execute(LeaveAlternateScreen)?;
            disable_raw_mode()?;
            break;
        }
    }

    Ok(())
}

#[cfg(not(feature = "console"))]
fn initialize_logging() -> Result<()> {
    if std::env::var("RUST_LOG").is_ok() {
        let log_file = std::fs::File::create("./probe.log")?;
        let file_subscriber = tracing_subscriber::fmt::layer()
            .with_file(true)
            .with_line_number(true)
            .with_writer(log_file)
            .with_target(false)
            .with_ansi(false)
            .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

        tracing_subscriber::registry()
            .with(file_subscriber)
            .with(ErrorLayer::default())
            .init();
    }

    Ok(())
}

#[cfg(feature = "console")]
fn initialize_logging() -> Result<()> {
    if std::env::var("RUST_LOG").is_ok() {
        let log_file = std::fs::File::create("./probe.log")?;
        let file_subscriber = tracing_subscriber::fmt::layer()
            .with_file(true)
            .with_line_number(true)
            .with_writer(log_file)
            .with_target(false)
            .with_ansi(false)
            .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());
        let console_subscriber = console_subscriber::spawn();

        tracing_subscriber::registry()
            .with(file_subscriber)
            .with(console_subscriber)
            .with(ErrorLayer::default())
            .init();
    }

    Ok(())
}
