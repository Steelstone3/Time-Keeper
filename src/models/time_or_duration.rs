use chrono::{Duration, NaiveTime, TimeDelta};

#[derive(Debug, PartialEq, Eq)]
pub struct TimeOrDuration {
    pub time: Option<NaiveTime>,
    pub duration: Option<TimeDelta>,
}

impl TimeOrDuration {
    pub fn new_time(time: NaiveTime) -> Self {
        Self {
            time: Some(time),
            duration: None,
        }
    }

    pub fn new_duration(duration: TimeDelta) -> TimeOrDuration {
        Self {
            time: None,
            duration: Some(duration),
        }
    }
}
