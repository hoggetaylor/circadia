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

pub struct FutureSunEvents {
    pos: GlobalPosition,
    current_time: DateTime<Utc>,
    event_whitelist_iter: Cycle<VecIter<CycleState<SunEvent>>>
}

impl FutureSunEvents {

    pub fn starting_from(start_date: DateTime<Utc>, pos: GlobalPosition, event_whitelist: &[SunEvent]) -> Self {
        assert!(!event_whitelist.is_empty());

        let mut events = event_whitelist.to_owned();
        events.sort();
        let mut cycled_events = vec![];
        for event in events {
            cycled_events.push(CycleState::Next(event));
        }
        cycled_events.push(CycleState::Restarting);

        FutureSunEvents {
            pos,
            current_time: start_date,
            event_whitelist_iter: cycled_events.into_iter().cycle()
        }
    }

}

impl Iterator for FutureSunEvents {

    type Item = (SunEvent, DateTime<Utc>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let CycleState::Next(event) = self.event_whitelist_iter.next().unwrap() {
                if let Some(event_time) = time_of_event(self.current_time.date(), &self.pos, event) {
                    if event_time > self.current_time {
                        self.current_time = event_time;
                        return Some((event, event_time));
                    }
                }
            } else {
                let tomorrow = self.current_time.date().succ();
                self.current_time = tomorrow.and_hms(0, 0, 0);
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
        let events = FutureSunEvents::starting_from(Utc::now(), pos, whitelist);
        for (event, _time) in events.take(500) {
            assert!(event == SunEvent::SUNRISE || event == SunEvent::SUNSET);
        }
    }

    #[test]
    fn should_never_skip_a_day() {
        let pos = GlobalPosition::at(40.60710285372043, -111.85515699873065);
        let whitelist = &[SunEvent::SUNRISE];
        let events = FutureSunEvents::starting_from(Utc::now(), pos, whitelist);
        let mut maybe_last_time: Option<DateTime<Utc>> = None;
        for (_event, time) in events.take(500) {
            if let Some(last_time) = maybe_last_time {
                assert!(last_time.date().succ() == time.date());
            };
            maybe_last_time = Some(time);
        }
    }

}
