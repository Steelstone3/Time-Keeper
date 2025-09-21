use crate::models::{time_keeper::TimeKeeper, time_or_duration::TimeOrDuration};
use chrono::NaiveTime;

impl TimeKeeper {
    pub fn calculate_time(&self, times: Vec<TimeOrDuration>, operations: char) -> String {
        match operations {
            '+' => todo!(),
            '-' => todo!(),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod time_calculator_should {
    use crate::models::{time_keeper::TimeKeeper, time_or_duration::TimeOrDuration};
    use chrono::{Duration, NaiveTime};
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[ignore = "meh"]
    #[case("14:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap())], ' ', "14:00" )]
    #[ignore = "meh"]
    #[case("14:00 - 13:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("13:00").unwrap())], '-', "1 Hour/s From Now" )]
    #[ignore = "meh"]
    #[case("13:00 - 14:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("13:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap())], '-', "1 Hour/s In The Past" )]
    #[ignore = "meh"]
    #[case("13:00 + 6", vec![TimeOrDuration::new_time(NaiveTime::from_str("13:00").unwrap()), TimeOrDuration::new_duration(Duration::hours(6))], '+', "6 Hours/s From Now Is 19:00" )]
    #[ignore = "meh"]
    #[case("13:00 + 6.5", vec![TimeOrDuration::new_time(NaiveTime::from_str("13:00").unwrap()), TimeOrDuration::new_duration(Duration::hours(6))], '+', "6.5 Hours/s From Now Is 19:30" )]
    fn parse_time(
        #[case] time_string: String,
        #[case] times: Vec<TimeOrDuration>,
        #[case] operation: char,
        #[case] expected_result_string: String,
    ) {
        // Given
        let time_keeper = TimeKeeper {
            time_calculation_string: time_string,
            ..Default::default()
        };

        // When
        let result = time_keeper.calculate_time(times, operation);

        // Then
        assert_eq!(expected_result_string, result)
    }
}
