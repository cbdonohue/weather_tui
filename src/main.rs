//! This module handles the fetching and displaying of weather information
//! for a specified location using a terminal UI.

mod weather; // Import the weather module

use std::io::{self, stdout};
use weather::WeatherResponse; // Use WeatherResponse struct

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};

/// The entry point of the application.
#[tokio::main]
async fn main() -> io::Result<()> {
    // Set up the terminal environment for the application
    setup_terminal()?;

    // Specify the latitude and longitude for the desired location
    let lat = 40.7128; // Latitude for NYC
    let lon = -74.0060; // Longitude for NYC

    // Fetch the weather data for the specified location
    let weather_data = match weather::fetch_weather(lat, lon).await {
        Ok(data) => Some(data),
        Err(e) => {
            eprintln!("Error fetching weather: {:?}", e);
            None
        }
    };

    // Run the main application loop
    run_application(weather_data)?;

    // Reset the terminal to its default state
    reset_terminal()
}

/// Configures the terminal for rendering the UI.
fn setup_terminal() -> io::Result<()> {
    enable_raw_mode()?; // Enable raw mode to capture input events directly
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

/// Resets the terminal to its normal state after exiting the application.
fn reset_terminal() -> io::Result<()> {
    disable_raw_mode()?; // Disable raw mode to restore normal terminal behavior
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

/// Runs the main application loop, rendering the UI and handling events.
///
/// # Arguments
///
/// * `weather_data` - An Option containing the WeatherResponse to display.
fn run_application(weather_data: Option<WeatherResponse>) -> io::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|f| ui(f, &weather_data))?; // Pass the WeatherResponse to the UI function
        should_quit = handle_events()?; // Handle input events and check for exit condition
    }

    Ok(())
}

/// Handles input events and determines whether to exit the application.
///
/// # Returns
///
/// A Result indicating whether the application should quit.
fn handle_events() -> io::Result<bool> {
    // Poll for input events with a timeout of 50 milliseconds
    if event::poll(std::time::Duration::from_millis(50))? {
        // Read the event and check if it's a key press event
        if let Event::Key(key) = event::read()? {
            // Check if the 'q' key was pressed to exit the application
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true); // Return true to indicate that the application should quit
            }
        }
    }
    Ok(false) // Return false to continue the application loop
}

/// Renders the UI frame, displaying the weather information.
///
/// # Arguments
///
/// * `frame` - The frame to render the UI into.
/// * `weather_data` - An Option containing the WeatherResponse to display.
fn ui(frame: &mut Frame, weather_data: &Option<WeatherResponse>) {
    // Generate the weather information string
    let weather_info = match weather_data {
        Some(weather) => {
            let temp = weather.current.temperature_2m;
            format!("Temperature: {:.1}°F", temp)
        }
        None => "No weather data available".to_string(),
    };

    // Render the weather information as a paragraph widget within a bordered block
    frame.render_widget(
        Paragraph::new(weather_info).block(Block::bordered().title("Weather Info")),
        frame.size(),
    );
}
