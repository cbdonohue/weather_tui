use std::io::{self, stdout};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use open_meteo_rs::forecast::{ForecastResult, Options};
use serde_json::Value; // Import serde_json for JSON handling
use log::{info, warn, error};
use simplelog::{Config, WriteLogger, LevelFilter};
use std::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize logging with simplelog
    WriteLogger::init(
        LevelFilter::Info, 
        Config::default(), 
        File::create("app.log").unwrap()
    ).unwrap();

    info!("Starting the application...");

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let client = open_meteo_rs::Client::new();
    let mut opts = Options::default();

    // Set location and options
    opts.location = open_meteo_rs::Location {
        lat: 48.864716,
        lng: 2.349014,
    };

    opts.current.push("temperature_2m".into());

    // Fetch the forecast
    info!("Fetching weather forecast...");
    let res: ForecastResult = match client.forecast(opts).await {
        Ok(forecast) => {
            info!("Forecast successfully retrieved: {:#?}", forecast);
            forecast
        }
        Err(e) => {
            error!("Failed to fetch forecast: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Forecast retrieval failed"));
        }
    };

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| ui(f, &res))?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    info!("Application exiting...");
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                info!("Received quit command.");
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, res: &ForecastResult) {
    // Convert the response to a JSON Value
    let json: Value = serde_json::to_value(&res).unwrap();

    // Pretty print the JSON
    let pretty_json = serde_json::to_string_pretty(&json).unwrap();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Weather Forecast");

    let paragraph = Paragraph::new(pretty_json).block(block);

    frame.render_widget(paragraph, frame.size());
}
