use std::env;
use std::io::{self, stdout};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
    style::{Color, Style},
    symbols,
};
use open_meteo_rs::forecast::{ForecastResult, Options, TemperatureUnit};
use log::{info, error, debug};
use simplelog::{Config, WriteLogger, LevelFilter};
use std::fs::File;
use chrono::NaiveDate;

/// Main entry point of the application.
///
/// Sets up the terminal interface, initializes logging, and fetches weather forecast data.
/// The forecast data is then displayed as a chart in the terminal. The user can specify a location
/// via command line arguments, or it defaults to New York City if no location is provided.
///
/// # Errors
///
/// This function will return an `io::Error` if there is an issue with terminal operations
/// or if fetching the weather forecast fails.
#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize logging with simplelog
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("app.log").unwrap(),
    )
    .unwrap();

    info!("Starting the application...");

    // Parse command line arguments for location
    let args: Vec<String> = env::args().collect();
    let (lat, lng) = if args.len() == 3 {
        let lat = args[1].parse().unwrap_or(40.7128); // Default to NYC latitude if parse fails
        let lng = args[2].parse().unwrap_or(-74.0060); // Default to NYC longitude if parse fails
        (lat, lng)
    } else {
        (40.7128, -74.0060) // Default to New York City coordinates
    };

    info!("Using location: Latitude {}, Longitude {}", lat, lng);

    // Enable raw mode for the terminal to capture input events
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let client = open_meteo_rs::Client::new();
    let mut opts = Options::default();

    // Set location and options
    opts.location = open_meteo_rs::Location { lat, lng };
    opts.forecast_days = Some(10);

    // Set temperature unit to Fahrenheit
    opts.temperature_unit = Some(TemperatureUnit::Fahrenheit);

    // Request maximum daily temperature
    opts.daily.push("temperature_2m_max".into());

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

    // Restore the terminal state
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    info!("Application exiting...");
    Ok(())
}

/// Handles terminal input events.
///
/// Listens for keyboard input and checks if the 'q' key is pressed to exit the application.
///
/// # Returns
///
/// Returns `Ok(true)` if the 'q' key is pressed, indicating that the application should quit.
/// Otherwise, returns `Ok(false)`.
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

/// Extracts daily maximum temperatures from the forecast result.
///
/// # Arguments
///
/// * `forecast` - A reference to the `ForecastResult` containing daily weather data.
///
/// # Returns
///
/// Returns a vector of tuples where each tuple contains a `NaiveDate` and a `f64` representing
/// the date and maximum temperature for that day, respectively.
fn extract_temperature_data(forecast: &ForecastResult) -> Vec<(NaiveDate, f64)> {
    if let Some(daily) = &forecast.daily {
        daily
            .iter()
            .map(|entry| {
                let temp = entry
                    .values
                    .get("temperature_2m_max")
                    .and_then(|v| v.value.as_f64())
                    .unwrap_or(0.0);
                (entry.date, temp)
            })
            .collect()
    } else {
        vec![]
    }
}

/// Renders the user interface and displays the weather forecast chart.
///
/// # Arguments
///
/// * `frame` - A mutable reference to the `Frame` used for rendering.
/// * `res` - A reference to the `ForecastResult` containing the weather data.
fn ui(frame: &mut Frame, res: &ForecastResult) {
    // Extract temperature data with dates
    let temp_data = extract_temperature_data(res);

    // Log temperature data
    debug!("Temperature data: {:?}", temp_data);

    // Prepare data for the chart
    let chart_data: Vec<(f64, f64)> = temp_data
        .iter()
        .enumerate()
        .map(|(i, &(_, temp))| (i as f64, temp))
        .collect();

    // Prepare x-axis labels with dates
    let x_labels: Vec<String> = temp_data
        .iter()
        .map(|(date, _)| date.format("%m/%d").to_string())
        .collect();

    // Create the datasets to fill the chart with
    let datasets = vec![Dataset::default()
        .name("High")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Magenta))
        .data(&chart_data)];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .style(Style::default().fg(Color::White))
        .bounds([0.0, chart_data.len() as f64])
        .labels(x_labels.iter().map(|s| s.into()).collect());

    // Calculate y-axis bounds dynamically
    let (min_temp, max_temp) = temp_data.iter().fold((f64::MAX, f64::MIN), |(min, max), &(_, temp)| {
        (min.min(temp), max.max(temp))
    });

    // Add a margin for better visualization
    let margin = 5.0;
    let y_bounds = [min_temp - margin, max_temp + margin];

    // Generate y-axis labels dynamically
    let y_labels: Vec<String> = (y_bounds[0] as i32..=y_bounds[1] as i32)
        .step_by(5) // Adjust step size for better label distribution
        .map(|v| format!("{}°F", v))
        .collect();

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .style(Style::default().fg(Color::White))
        .bounds(y_bounds) // Adjust bounds to reflect Fahrenheit range
        .labels(y_labels.iter().map(|s| s.into()).collect());

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::default().title("Forecast").borders(Borders::ALL))
        .x_axis(x_axis)
        .y_axis(y_axis);

    // Render the chart
    frame.render_widget(chart, frame.size());
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_meteo_rs::forecast::{ForecastResult, ForecastResultItem};
    use std::collections::HashMap;
    use chrono::NaiveDate;

    /// Tests the `extract_temperature_data` function to ensure it correctly extracts temperatures.
    #[test]
    fn test_extract_temperature_data() {
        // Create sample forecast data
        let forecast = ForecastResult {
            current: None,
            hourly: None,
            daily: Some(vec![
                ForecastResultDaily {
                    date: NaiveDate::parse_from_str("2024-08-02", "%Y-%m-%d").unwrap(),
                    values: {
                        let mut map = HashMap::new();
                        map.insert(
                            "temperature_2m_max".into(),
                            ForecastResultItem {
                                unit: Some("°F".into()),
                                value: 82.76.into(), // Example Fahrenheit value
                            },
                        );
                        map
                    },
                },
                ForecastResultDaily {
                    date: NaiveDate::parse_from_str("2024-08-03", "%Y-%m-%d").unwrap(),
                    values: {
                        let mut map = HashMap::new();
                        map.insert(
                            "temperature_2m_max".into(),
                            ForecastResultItem {
                                unit: Some("°F".into()),
                                value: 75.2.into(), // Example Fahrenheit value
                            },
                        );
                        map
                    },
                },
                // Add more sample data if needed
            ]),
        };

        // Expected output
        let expected = vec![
            (
                NaiveDate::parse_from_str("2024-08-02", "%Y-%m-%d").unwrap(),
                82.76,
            ),
            (
                NaiveDate::parse_from_str("2024-08-03", "%Y-%m-%d").unwrap(),
                75.2,
            ),
        ];

        // Call the function
        let result = extract_temperature_data(&forecast);

        // Assert the result matches the expected output
        assert_eq!(result, expected);
    }
}
