use chrono::{Duration, TimeDelta};

// TODO AH Test
pub fn duration_factory(duration: &str) -> Result<TimeDelta, ()> {
    if duration.contains(':') {
        let times: Vec<&str> = duration.split(':').collect();

        if times.len() != 2 {
            return Err(());
        }

        let hours: i64 = match times[0].parse() {
            Ok(hours) => hours,
            Err(_) => {
                return Err(());
            }
        };

        let minutes: i64 = match times[1].parse() {
            Ok(minutes) => minutes,
            Err(_) => return Err(()),
        };

        Ok(Duration::hours(hours) + Duration::minutes(minutes))
    } else {
        let hours: f32 = match duration.parse() {
            Ok(hours) => hours,
            Err(_) => return Err(()),
        };

        let the_hours = hours.trunc() as i64;
        let minutes = (60.0 * hours.fract()) as i64;

        Ok(Duration::hours(the_hours) + Duration::minutes(minutes))
    }
}

#[cfg(test)]
mod duration_factory_should {}
