use serde::{Deserialize, Serialize};
use reqwest::Error; // Ensure this import is here for the Error type

/// Represents the units for the current weather data.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct CurrentUnits {
    /// The time of the weather data.
    pub time: String,
    /// The interval of the weather data.
    pub interval: String,
    /// The unit for temperature at 2 meters above ground.
    pub temperature_2m: String,
    /// The unit for wind speed at 10 meters above ground.
    pub wind_speed_10m: String,
}

/// Represents the current weather data.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Current {
    /// The time of the weather data.
    pub time: String,
    /// The interval of the weather data in minutes.
    pub interval: u32,
    /// The temperature at 2 meters above ground in Celsius.
    pub temperature_2m: f64,
    /// The wind speed at 10 meters above ground in meters per second.
    pub wind_speed_10m: f64,
}

/// Represents the weather response from the API.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct WeatherResponse {
    /// The latitude of the location.
    pub latitude: f64,
    /// The longitude of the location.
    pub longitude: f64,
    /// The timezone of the location.
    pub timezone: String,
    /// The abbreviation for the timezone.
    pub timezone_abbreviation: String,
    /// The elevation of the location in meters.
    pub elevation: f64,
    /// The units for the current weather data.
    pub current_units: CurrentUnits,
    /// The current weather data.
    pub current: Current,
}

/// Fetches the current weather data for a given latitude and longitude.
///
/// # Arguments
///
/// * `lat` - The latitude of the location.
/// * `lon` - The longitude of the location.
///
/// # Returns
///
/// * `Ok(WeatherResponse)` - The weather data if successful.
/// * `Err(Error)` - An error if the request fails.
///
/// # Examples
///
/// ```rust,no_run
/// use weather_tui::weather::fetch_weather;
///
/// #[tokio::main]
/// async fn main() {
///     let lat = 40.7128; // latitude for NYC
///     let lon = -74.0060; // longitude for NYC
///
///     match fetch_weather(lat, lon).await {
///         Ok(weather) => println!("Weather data: {:?}", weather),
///         Err(e) => eprintln!("Error fetching weather: {:?}", e),
///     }
/// }
/// ```
pub async fn fetch_weather(lat: f64, lon: f64) -> Result<WeatherResponse, Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m&temperature_unit=fahrenheit",
        lat, lon
    );
    let response = reqwest::get(&url).await?;
    let weather_data = response.json::<WeatherResponse>().await?;
    Ok(weather_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_weather() {
        // Mocked data or a real API call can be used here.
        // Here, we use a known location for testing purposes.
        let lat = 40.7128;
        let lon = -74.0060;

        let result = fetch_weather(lat, lon).await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_weather_response_struct() {
        // Example data to test struct deserialization
        let json_data = r#"
        {
            "latitude": 40.7128,
            "longitude": -74.0060,
            "timezone": "America/New_York",
            "timezone_abbreviation": "EDT",
            "elevation": 10.0,
            "current_units": {
                "time": "2024-08-01T00:00:00Z",
                "interval": "1h",
                "temperature_2m": "°C",
                "wind_speed_10m": "m/s"
            },
            "current": {
                "time": "2024-08-01T00:00:00Z",
                "interval": 60,
                "temperature_2m": 25.0,
                "wind_speed_10m": 5.0
            }
        }
        "#;

        let weather_response: WeatherResponse = serde_json::from_str(json_data).unwrap();

        assert_eq!(weather_response.latitude, 40.7128);
        assert_eq!(weather_response.longitude, -74.0060);
        assert_eq!(weather_response.timezone, "America/New_York");
        assert_eq!(weather_response.current.temperature_2m, 25.0);
    }
}
