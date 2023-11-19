//! # linux_max6675
//!
//! A library that helps you read from a MAX6675 over Linux SPI.
//!
//! ## Usage
//!
//! To use this library, you'll need to know which SPI device to select.
//! On Linux, you can use `ls /dev -1 | grep spidev` to figure it out!
//!
//! Then, you can use something like this example in your binary...
//!
//! ```no_run
//! use linux_max6675::Max6675;
//! use std::time::Duration;
//!
//! let mut max = Max6675::new("/dev/spidev0.0")?;
//!
//! std::thread::sleep(Duration::from_secs(3));
//!
//! loop {
//!     println!("Read Celsius! Got: {}° C.", max.read_celsius()?.into_inner());
//!     std::thread::sleep(Duration::from_millis(500));
//! }
//! ```

use connection::Connection;
use temperature::Temperature;
use thiserror::Error;

pub mod connection;
pub mod temperature;

#[derive(Debug, Error)]
pub enum Max6675Error {
    #[error("We couldn't connect to the provided SPI path. See std::io::Error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("The MAX6675 detected an open circuit (bit D2 was high). Please check the thermocouple connection and try again.")]
    OpenCircuitError,
}

#[derive(Debug)]
pub struct Max6675 {
    connection: Connection,
}

impl Max6675 {
    pub fn new(spi_connection_path: impl AsRef<str>) -> Result<Self, Max6675Error> {
        Ok(Self {
            connection: Connection::new(spi_connection_path)?,
        })
    }

    /// Reads the thermocouple's temperature in Celsius.
    pub fn read_celsius(&mut self) -> Result<Temperature, Max6675Error> {
        Ok(Temperature::Celsius(self.connection.read_as_celsius()?))
    }

    /// Reads the thermocouple's temperature in Fahrenheit.
    pub fn read_fahrenheit(&mut self) -> Result<Temperature, Max6675Error> {
        Ok(self.read_celsius()?.to_fahrenheit())
    }

    /// Reads the thermocouple's temperature in Kelvin.
    pub fn read_kelvin(&mut self) -> Result<Temperature, Max6675Error> {
        Ok(self.read_celsius()?.to_kelvin())
    }

    /// Returns the thermocouple's raw data for data science. (and other fun little things)
    ///
    /// Refer to page 5 of [Maxim Integrated's MAX6675 specsheet](https://www.analog.com/media/en/technical-documentation/data-sheets/MAX6675.pdf)
    /// for info on how to interpret this raw data.
    pub fn read_raw(&mut self) -> Result<[u8; 2], Max6675Error> {
        self.connection.read_raw()
    }
}
