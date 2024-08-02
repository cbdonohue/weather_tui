
# Weather TUI

![image](https://github.com/user-attachments/assets/1095f55b-d724-4149-8839-8121363e36a5)

Weather TUI is a terminal-based weather application built in Rust that displays a forecast using a text-based user interface. It fetches weather data from the Open Meteo API and visualizes daily maximum temperatures on a chart. By default, it shows the forecast for New York City, but you can specify any location using command line arguments.

## Features

- **Command Line Location:** Specify a location using latitude and longitude.
- **Default Location:** Defaults to New York City if no location is specified.
- **Weather Visualization:** Displays a chart of daily maximum temperatures.
- **Interactive Terminal UI:** Navigate the application using terminal controls.
- **Logging:** Logs application activity to a file for easy debugging.

## Getting Started

### Prerequisites

To run this application, you need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)
- Internet connection for fetching weather data

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/weather-tui.git
   cd weather-tui
   ```

2. Build the application:

   ```bash
   cargo build --release
   ```

3. Run the application:

   ```bash
   cargo run --release
   ```

   You can optionally provide latitude and longitude to get the forecast for a specific location:

   ```bash
   cargo run --release -- 34.0522 -118.2437
   ```

   This example fetches the forecast for Los Angeles, CA.

## Usage

- **Navigation:** 
  - Use your terminal to view the weather chart.
  - Press `q` to exit the application.

- **Logging:**
  - Application logs are saved to `app.log` in the root directory.

## Code Structure

- **Main Application (`main.rs`):** 
  - Handles terminal initialization, command line parsing, and fetching the weather forecast.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

