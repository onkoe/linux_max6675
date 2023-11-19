# linux_max6675

This is a Rust library that helps you communicate with your MAX6675 over Linux's SPI API.

## Usage

For full examples, please see [the examples folder](https://github.com/onkoe/linux_max6675/blob/examples/duo.rs)!

However, you can get away with using something like this:

```rust
let mut max = Max6675::new("/dev/spidev0.0")?;

std::thread::sleep(Duration::from_secs(3));

loop {
    println!("Read Celsius! Got: {}° C.", max.read_celsius()?.into_inner());
    std::thread::sleep(Duration::from_millis(500));
}
```

## Why..?

I built this library for use on my robotics and vehicular telemetry projects. Please let me know if there are any missing features - I'm happy to add them. 🤩️

I'd also like to see additional examples. If you have any ideas, please let me know and I'll stick it in the `examples` folder.

Also, it's fairly likely this'll end up on [my blog](https://barretts.club) sometime soon! If this is looking a bit difficult to use, I'll definitely have a simple guide up at some point!
