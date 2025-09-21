use chrono::{Duration, TimeDelta};

pub fn duration_factory(duration: &str) -> Result<TimeDelta, ()> {
    if duration.contains('-') {
        return Err(());
    }

    if duration.contains(':') {
        let times: Vec<&str> = duration.split(':').collect();

        if times.len() != 2 {
            return Err(());
        }

        let hours: i64 = match times[0].parse() {
            Ok(hours) => {
                if hours >= 24 {
                    return Err(());
                }

                hours
            }
            Err(_) => {
                return Err(());
            }
        };

        let minutes: i64 = match times[1].parse() {
            Ok(minutes) => {
                if minutes >= 60 {
                    return Err(());
                }

                minutes
            }
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

        if the_hours >= 24 {
            return Err(());
        }

        if minutes >= 60 {
            return Err(());
        }

        Ok(Duration::hours(the_hours) + Duration::minutes(minutes))
    }
}

#[cfg(test)]
mod duration_factory_should {
    use crate::controllers::duration_factory::duration_factory;
    use chrono::{Duration, TimeDelta};
    use rstest::rstest;

    #[rstest]
    #[case("1", Duration::hours(1))]
    #[case("10", Duration::hours(10))]
    #[case("23", Duration::hours(23))]
    #[case("1:1", Duration::hours(1) + Duration::minutes(1))]
    #[case("1:59", Duration::hours(1) + Duration::minutes(59))]
    #[case("23:59", Duration::hours(23) + Duration::minutes(59))]
    fn create(#[case] duration: String, #[case] expected_duration_result: TimeDelta) {
        // When
        let duration_result = duration_factory(&duration);

        // Then
        assert_eq!(expected_duration_result, duration_result.unwrap())
    }

    #[rstest]
    #[case("asdsad")]
    #[case("12:xx")]
    #[case("xx:12")]
    #[case("::")]
    #[case("-23")]
    #[case("-23:00")]
    #[case("23:60")]
    #[case("24")]
    #[case("24:00")]
    #[should_panic]
    fn create_invalid(#[case] duration: String) {
        // When
        let _ = duration_factory(&duration).unwrap();
    }
}
