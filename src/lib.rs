
//! This crate provides a simple interface to compute information about
//! the sunrise and sunset on arbitrary dates at any position
//! on the earth.

mod event;
mod pos;
mod algorithm;
mod iter;

pub use event::{ Event, Zenith, SunEvent };
pub use pos::GlobalPosition;
pub use algorithm::time_of_event;
pub use iter::{ SunEvents, ForecastedSunEvents, HistoricSunEvents };
