
//! This module provides types for representing
//! a position on the globe.

/// Represents a position on the earth.
#[derive(Debug, Clone)]
pub struct GlobalPosition {
    latitude: f64,
    longitude: f64,
    lng_hour: f64
}

impl GlobalPosition {

    /// Create a new GlobalPosition at the
    /// given latitude and longitude
    pub fn at(lat: f64, lng: f64) -> Self {
        GlobalPosition {
            latitude: lat,
            longitude: lng,
            lng_hour: lng / 15.0
        }
    }

    /// The latitude of the position
    pub fn lat(&self) -> f64 {
        self.latitude
    }

    /// The longitude of the position
    pub fn lng(&self) -> f64 {
        self.longitude
    }

    pub(crate) fn lng_hour(&self) -> f64 {
        self.lng_hour
    }

    /// Returns a [FixedOffset] timezone calculated from
    /// this location's longitude
    ///
    /// [FixedOffset]: chrono::FixedOffset
    pub fn lng_timezone(&self) -> chrono::FixedOffset {
        const SECS_IN_HOUR: f64 = 3600_f64;
        if self.lng() >= 0_f64 {
            chrono::FixedOffset::east((self.lng_hour.abs() * SECS_IN_HOUR) as i32)
        } else {
            chrono::FixedOffset::west((self.lng_hour.abs() * SECS_IN_HOUR) as i32)
        }
    }

}
