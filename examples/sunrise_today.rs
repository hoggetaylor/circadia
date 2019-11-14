use circadia::{ GlobalPosition, SunEvent, time_of_event };
use chrono::Utc;

fn main() {
    // Nauticalia Greenwhich
    let pos = GlobalPosition::at(51.4810066, 0.0081805);
    let today = Utc::now().date();
    let sunrise_time = time_of_event(today, &pos, SunEvent::SUNRISE).unwrap();
    println!("Time of sunrise today: {}", sunrise_time.format("%r"));
}