use chrono::{ DateTime, Utc };
use std::iter::Cycle;
use std::vec::IntoIter as VecIter;
use super::event::SunEvent;
use super::time_of_event;
use super::pos::GlobalPosition;

#[derive(Debug, Clone)]
enum CycleState<T> {
    Next(T),
    Restarting
}

fn cycled(events: &[SunEvent]) -> Cycle<VecIter<CycleState<SunEvent>>> {
    assert!(!events.is_empty());
    let mut events = events.to_owned();
    events.sort();
    events.dedup();
    let mut cycled_events = vec![];
    for event in events {
        cycled_events.push(CycleState::Next(event));
    }
    cycled_events.push(CycleState::Restarting);
    cycled_events.into_iter().cycle()
}

/// This struct allows one to create iterators over sun events moving
/// forward or backward in time.
#[derive(Debug, Clone)]
pub struct SunEvents {
    pos: GlobalPosition,
    current_time: DateTime<Utc>,
    event_whitelist_iter: Cycle<VecIter<CycleState<SunEvent>>>
}

impl SunEvents {

    /// List SunEvents starting from the `start_date`, computed at `position`,
    /// including only the SunEvents listed in the `event_whitelist`.
    /// # Panics
    /// Panics when `event_whitelist` is empty.
    pub fn starting_from(start_date: DateTime<Utc>, position: GlobalPosition, event_whitelist: &[SunEvent]) -> Self {
        SunEvents {
            pos: position,
            current_time: start_date,
            event_whitelist_iter: cycled(event_whitelist)
        }
    }

    /// List SunEvents occurring after the start_date.
    pub fn forecast(self) -> ForecastedSunEvents {
        ForecastedSunEvents(self)
    }

    /// List SunEvents occurring before the start_date.
    pub fn history(self) -> HistoricSunEvents {
        HistoricSunEvents(self)
    }

}

/// An iterator that yields SunEvents that occur after
/// a specified start date.
pub struct ForecastedSunEvents(SunEvents);

impl Iterator for ForecastedSunEvents {

    type Item = (SunEvent, DateTime<Utc>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let CycleState::Next(event) = self.0.event_whitelist_iter.next().unwrap() {
                if let Some(event_time) = time_of_event(self.0.current_time.date(), &self.0.pos, event) {
                    if event_time > self.0.current_time {
                        self.0.current_time = event_time;
                        return Some((event, event_time));
                    }
                }
            } else {
                let tomorrow = self.0.current_time.date().succ();
                self.0.current_time = tomorrow.and_hms(0, 0, 0);
            }
        }
    }

}

/// An iterator that yields SunEvents that occur before
/// a specified start date.
pub struct HistoricSunEvents(SunEvents);

impl Iterator for HistoricSunEvents {

    type Item = (SunEvent, DateTime<Utc>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let CycleState::Next(event) = self.0.event_whitelist_iter.next().unwrap() {
                if let Some(event_time) = time_of_event(self.0.current_time.date(), &self.0.pos, event) {
                    if event_time < self.0.current_time {
                        self.0.current_time = event_time;
                        return Some((event, event_time));
                    }
                }
            } else {
                let yesterday = self.0.current_time.date().pred();
                self.0.current_time = yesterday.and_hms(23, 59, 59);
            }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_only_produce_events_in_the_whitelist() {
        let pos = GlobalPosition::at(70.0, 34.0);
        let whitelist = &[SunEvent::SUNRISE, SunEvent::SUNSET];
        let events = SunEvents::starting_from(Utc::now(), pos, whitelist);
        for (event, _time) in events.forecast().take(500) {
            assert!(event == SunEvent::SUNRISE || event == SunEvent::SUNSET);
        }
    }

    #[test]
    fn forecast_should_never_skip_a_day() {
        let pos = GlobalPosition::at(40.60710285372043, -111.85515699873065);
        let whitelist = &[SunEvent::SUNRISE];
        let events = SunEvents::starting_from(Utc::now(), pos, whitelist);
        let mut maybe_last_time: Option<DateTime<Utc>> = None;
        for (_event, time) in events.forecast().take(500) {
            if let Some(last_time) = maybe_last_time {
                assert_eq!(last_time.date().succ(), time.date());
            };
            maybe_last_time = Some(time);
        }
    }

    #[test]
    fn history_should_never_skip_a_day() {
        let pos = GlobalPosition::at(40.60710285372043, -111.85515699873065);
        let whitelist = &[SunEvent::SUNRISE];
        let events = SunEvents::starting_from(Utc::now(), pos, whitelist);
        let mut maybe_last_time: Option<DateTime<Utc>> = None;
        for (_event, time) in events.history().take(500) {
            if let Some(last_time) = maybe_last_time {
                assert_eq!(last_time.date().pred(), time.date());
            };
            maybe_last_time = Some(time);
        }
    }

}
