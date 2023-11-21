//! # duo.rs
//!
//! This example works great on my Milk-V Duo!
//! It should serve as an excellent starting point for your own projects, too!

use std::time::Duration;

use linux_max6675::Max6675;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!("Hello, world!");

    // assuming your MAX6675 is found at this path
    let mut max = Max6675::new("/dev/spidev0.0")?;

    // sleep for a moment, just to let everything 'settle'
    std::thread::sleep(Duration::from_secs(3));

    loop {
        tracing::info!(
            "Read Celsius! Got: {}° C.",
            max.read_celsius()?
        );

        // you might want to sleep so you don't get double readings. feel free to play with this, though!
        std::thread::sleep(Duration::from_millis(500));
    }
}
