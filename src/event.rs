
//! This module provides types for representing
//! exact sunrise/sunset events.

use std::fmt;
use std::cmp::Ordering;

/// Defines how the sunset/sunrise is measured in relation to the horizon.
/// See https://www.timeanddate.com/astronomy/different-types-twilight.html
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Zenith {
    Golden,
    Official,
    Civil,
    Nautical,
    Astronomical
}

impl Zenith {

    pub(crate) fn angle(self) -> f64 {
        use Zenith::*;
        match self {
            Golden => 80.0,
            Official => 90.0,
            Civil => 96.0,
            Nautical => 102.0,
            Astronomical => 108.0
        }
    }

}

impl fmt::Display for Zenith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Zenith::Golden => write!(f, "golden"),
            Zenith::Official => write!(f, "official"),
            Zenith::Civil => write!(f, "civil"),
            Zenith::Nautical => write!(f, "nautical"),
            Zenith::Astronomical => write!(f, "astronomical"),
        }
    }
}

/// Represents either the sunset or the sunrise.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Event {
    Sunrise,
    Sunset
}

impl Event {

    pub(crate) fn hour(self) -> f64 {
        use Event::*;
        match self {
            Sunrise => 6.0,
            Sunset => 18.0
        }
    }

}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Event::Sunrise => write!(f, "sunrise"),
            Event::Sunset => write!(f, "sunset"),
        }
    }
}

// Defines a sunset or sunrise at some angle above the horizon (the zenith).
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SunEvent {
    pub zenith: Zenith,
    pub event: Event
}

impl SunEvent {

    pub const DAWN: SunEvent = SunEvent::new(Zenith::Civil, Event::Sunrise);
    pub const DUSK: SunEvent = SunEvent::new(Zenith::Civil, Event::Sunset);
    pub const SUNRISE: SunEvent = SunEvent::new(Zenith::Official, Event::Sunrise);
    pub const SUNSET: SunEvent = SunEvent::new(Zenith::Official, Event::Sunset);

    pub const fn new(zenith: Zenith, event: Event) -> Self {
        SunEvent { zenith, event }
    }

    pub fn is_sunrise(self) -> bool {
        use Event::*;
        match self.event {
            Sunrise => true,
            Sunset => false
        }
    }

    pub fn is_sunset(self) -> bool {
        !self.is_sunrise()
    }

}

impl Ord for SunEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        use Event::*;
        match (self.event, other.event) {
            (Sunrise, Sunrise) => self.zenith.cmp(&other.zenith).reverse(),
            (Sunset, Sunset) => self.zenith.cmp(&other.zenith),
            (a, b) => a.cmp(&b)
        }
    }
}

impl PartialOrd for SunEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for SunEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Event::*;
        use Zenith::*;
        match (self.zenith, self.event) {
            (Civil, Sunrise) => write!(f, "dawn"),
            (Civil, Sunset) => write!(f, "dusk"),
            (Official, Sunrise) => write!(f, "sunrise"),
            (Official, Sunset) => write!(f, "sunset"),
            (z, e) => write!(f, "{} {}", z, e)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn sunrise_should_sort_before_sunset() {
        use Event::*;
        let mut events = vec![Sunrise, Sunset, Sunrise, Sunset, Sunset, Sunset, Sunrise];
        events.sort();
        assert_eq!(events, vec![Sunrise, Sunrise, Sunrise, Sunset, Sunset, Sunset, Sunset]);
    }

    #[test]
    fn zenith_should_sort_in_order_of_angle() {
        use Zenith::*;
        let mut zeniths = vec![Golden, Official, Golden, Civil, Astronomical, Nautical, Astronomical, Official];
        zeniths.sort();
        assert_eq!(zeniths, vec![Golden, Golden, Official, Official, Civil, Nautical, Astronomical, Astronomical]);
    }

    #[test]
    fn sun_event_should_sort_in_order_of_occurence() {
        let mut events = vec![SunEvent::DAWN, SunEvent::DUSK, SunEvent::SUNRISE, SunEvent::SUNSET];
        events.sort();
        assert_eq!(events, vec![SunEvent::DAWN, SunEvent::SUNRISE, SunEvent::SUNSET, SunEvent::DUSK]);
    }

}
