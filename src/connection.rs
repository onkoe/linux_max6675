use std::io::Read;

use spidev::{SpiModeFlags, Spidev, SpidevOptions};

use crate::Max6675Error;

/// A struct that represents an SPI connection. Assumes you're using Linux.
#[derive(Debug)]
pub struct Connection {
    spi: Spidev,
    data: [u8; 2],
}

impl Connection {
    pub(crate) fn new(spi_path: impl AsRef<str>) -> Result<Self, Max6675Error> {
        let mut spi = Spidev::open(spi_path.as_ref())?;

        spi.configure(
            &SpidevOptions::new()
                .bits_per_word(8)
                .max_speed_hz(1_000_000)
                .mode(SpiModeFlags::SPI_MODE_1)
                .build(),
        )?;

        Ok(Self {
            spi,
            data: [0_u8; 2],
        })
    }

    pub(crate) fn read_raw(&mut self) -> Result<[u8; 2], Max6675Error> {
        self.spi.read_exact(&mut self.data)?;

        Ok(self.data)
    }

    pub(crate) fn read_as_celsius(&mut self) -> Result<f64, Max6675Error> {
        self.read_raw()?;

        let raw = u16::from_be_bytes(self.data);

        // check for Bit D2 being high, indicating that the thermocouple input is open
        // (see MAX6675 datasheet, p. 5)
        if raw & 0x04 != 0 {
            return Err(Max6675Error::OpenCircuitError);
        }

        // ripped from the Arduino library (see: https://github.com/RobTillaart/MAX6675)
        // TODO: sanity check..?
        let temp = ((raw >> 3) & 0x1FFF) as f64 * 0.25_f64;
        Ok(temp)
    }
}
