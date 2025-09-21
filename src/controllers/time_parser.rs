use chrono::{Duration, NaiveTime};
use std::str::FromStr;

use crate::models::{time_keeper::TimeKeeper, time_or_duration::TimeOrDuration};

impl TimeKeeper {
    pub fn parse_times(&self) -> Vec<TimeOrDuration> {
        let mut times = vec![];

        let trimmed = self.time_calculation_string.trim();

        let normalized = trimmed.replace(['+', '-'], ",");

        let string_times: Vec<&str> = normalized.split(',').collect();

        for string_time in string_times {
            let string_time = string_time.trim();

            if string_time.contains(':') {
                if let Ok(time) = NaiveTime::from_str(string_time) {
                    let time = TimeOrDuration::new_time(time);
                    times.push(time);
                }
            } else {
                match string_time.parse() {
                    Ok(hours) => {
                        let duration = Duration::hours(hours);
                        let duration = TimeOrDuration::new_duration(duration);

                        times.push(duration);
                    }
                    Err(_) => {}
                }
            }
        }

        times
    }

    pub fn parse_operations(&self) -> Vec<char> {
        let mut operations: Vec<char> = vec![];

        let trimmed = self.time_calculation_string.trim();

        for time_string_character in trimmed.chars() {
            if time_string_character == '+' || time_string_character == '-' {
                operations.push(time_string_character);
            }
        }

        operations
    }

     pub fn parse_operation(&self) -> char {
        let trimmed = self.time_calculation_string.trim();

        for time_string_character in trimmed.chars() {
            if time_string_character == '+' || time_string_character == '-' {
                return time_string_character
            }
        }

        ' '
    }
}

#[cfg(test)]
mod time_parser_should {
    use crate::models::{time_keeper::TimeKeeper, time_or_duration::TimeOrDuration};
    use chrono::{Duration, NaiveTime};
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case("", vec![] )]
    #[case("14:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap())] )]
    #[case("14:00 + 15:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("15:00").unwrap())] )]
    #[case("14:00 - 15:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("15:00").unwrap())] )]
    #[case("14:00 + 15:00 + 16:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("15:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("16:00").unwrap())] )]
    #[case("14:00 + 15:00 - 16:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("15:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("16:00").unwrap())] )]
    #[case("14:00 - 15:00 + 16:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("15:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("16:00").unwrap())] )]
    #[case("14:00 - 15:00 - 16:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("15:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("16:00").unwrap())] )]
    #[case("14:00 - 6", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_duration(Duration::hours(6))] )]
    #[case("14:00 + 6", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_duration(Duration::hours(6))] )]
    #[case("14:00 + 6 - 6", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_duration(Duration::hours(6)), TimeOrDuration::new_duration(Duration::hours(6))] )]
    #[case("asdasd", vec![] )]
    #[case("asdasd + asdas234$%#$%@#dasd", vec![] )]
    fn parse_time(#[case] time_string: String, #[case] expected_parsed_times: Vec<TimeOrDuration>) {
        // Given
        let time_keeper = TimeKeeper {
            time_calculation_string: time_string,
            ..Default::default()
        };

        // When
        let parsed_times = time_keeper.parse_times();

        // Then
        for index in 0..expected_parsed_times.len() {
            assert_eq!(expected_parsed_times.get(index), parsed_times.get(index));
        }
    }

    #[rstest]
    #[case("", vec![] )]
    #[case("14:00", vec![] )]
    #[case("14:00 + 15:00", vec!['+'] )]
    #[case("14:00 - 15:00", vec!['-'] )]
    #[case("14:00 + 15:00 + 16:00", vec!['+','+'])]
    #[case("14:00 + 15:00 - 16:00", vec!['+','-'])]
    #[case("14:00 - 15:00 + 16:00", vec!['-','+'])]
    #[case("14:00 - 15:00 - 16:00", vec!['-','-'])]
    fn parse_operation(#[case] time_string: String, #[case] expected_parsed_operations: Vec<char>) {
        // Given
        let time_keeper = TimeKeeper {
            time_calculation_string: time_string,
            ..Default::default()
        };

        // When
        let parsed_operations = time_keeper.parse_operations();

        // Then
        for index in 0..expected_parsed_operations.len() {
            assert_eq!(
                expected_parsed_operations.get(index),
                parsed_operations.get(index)
            );
        }
    }
}
