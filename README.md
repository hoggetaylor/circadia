# Circadia

This crate provides a simple interface to compute information about
the sunrise and sunset times on arbitrary dates at any position on the globe.

# Installation

```shell
$ cargo add circadia --version 0.0.1
```

# Usage

```rust
use circadia::{ GlobalPosition, SunEvent, time_of_event };
use chrono::Utc;

fn main() {
    // Nauticalia Greenwhich
    let pos = GlobalPosition::at(51.4810066, 0.0081805);
    let today = Utc::now().date();
    let sunrise_time = time_of_event(today, &pos, SunEvent::SUNRISE).unwrap();
    println!("Time of sunrise today: {}", sunrise_time.format("%r"));
}
```