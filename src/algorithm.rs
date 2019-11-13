#![allow(non_snake_case)]

use super::event::SunEvent;
use super::pos::GlobalPosition;
use chrono::{ Date, DateTime, Utc, Datelike, NaiveTime };

const SECS_IN_HOUR: i32 = 3600;

/// Calculates the time of the sunrise/sunset on the given date
/// at the given position on the globe.
///
/// This is an implementation of the algorithm described by the
/// United states Naval Observatory
/// found here: http://edwilliams.org/sunrise_sunset_algorithm.htm
///
/// Returns None if the sun never sets/rises on that day
/// (ie if you're in the arctic).
pub fn time_of_event(
    mut date: Date<Utc>,
    pos: &GlobalPosition,
    event: SunEvent,
) -> Option<DateTime<Utc>> {
    let D = date.ordinal() as f64;
    let t = approximate_time(D, event, pos);
    let M = mean_anomaly(t);
    let L = true_longitude(M);
    let RA = right_ascension(L);
    let H = match local_hour_angle(L, pos, event) {
        Some(H) => H,
        None => return None,
    };
    let T = local_mean_time(H, RA, t);
    let UT = rem_euclid(T - pos.lng_hour(), 24.0);
    let time = NaiveTime::from_num_seconds_from_midnight((UT * SECS_IN_HOUR as f64) as u32, 0);

    let should_be_yesterday = pos.lng_hour() > 0.0 && UT > 12.0 && event.is_sunrise();
    let should_be_tomorrow = pos.lng_hour() < 0.0 && UT < 12.0 && event.is_sunset();
    if should_be_yesterday {
        date = date.pred();
    } else if should_be_tomorrow {
        date = date.succ();
    }

    date.with_timezone(&Utc)
        .and_time(time)
}

fn approximate_time(D: f64, event: SunEvent, pos: &GlobalPosition) -> f64 {
    D + ((event.event.hour() - pos.lng_hour()) / 24.0)
}

fn mean_anomaly(t: f64) -> f64 {
    (0.9856 * t) - 3.289
}

fn true_longitude(M: f64) -> f64 {
    let L =
        M + (1.916 * M.to_radians().sin()) + (0.020 * (2.0 * M).to_radians().sin()) + 282.634;
    rem_euclid(L, 360.0)
}

fn right_ascension(L: f64) -> f64 {
    let mut RA = (0.91764 * L.to_radians().tan()).atan().to_degrees();
    RA = rem_euclid(RA, 360.0);
    let LQuadrant = (L / 90.0).floor() * 90.0;
    let RAQuadrant = (RA / 90.0).floor() * 90.0;
    (RA + (LQuadrant - RAQuadrant)) / 15.0
}

fn local_hour_angle(L: f64, pos: &GlobalPosition, event: SunEvent) -> Option<f64> {
    let sinDec = 0.39782 * L.to_radians().sin();
    let cosDec = sinDec.asin().cos();
    let z = event.zenith.angle().to_radians();
    let cosH = (z.cos() - (sinDec * pos.lat().to_radians().sin()))
        / (cosDec * pos.lat().to_radians().cos());
    if cosH > 1.0 && event.is_sunrise() {
        // The sun never rises on this location on the specified date.
        return None;
    }
    if cosH < -1.0 && event.is_sunset() {
        // The sun never sets on this location on the specified date.
        return None;
    }
    let H = if event.is_sunrise() {
        360.0 - cosH.acos().to_degrees()
    } else {
        cosH.acos().to_degrees()
    };
    Some(H / 15.0)
}

fn local_mean_time(H: f64, RA: f64, t: f64) -> f64 {
    H + RA - (0.06571 * t) - 6.622
}

fn rem_euclid(lhs: f64, rhs: f64) -> f64 {
    let r = lhs % rhs;
    if r < 0.0 {
        r + rhs.abs()
    } else {
        r
    }
}