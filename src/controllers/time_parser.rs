use chrono::NaiveTime;
use std::str::FromStr;

use crate::models::duplicate_finder::TimeKeeper;

impl TimeKeeper {
    pub fn parse_times(&self) -> Vec<NaiveTime> {
        let mut times = vec![];

        let trimmed = self.time_calculation_string.trim();

        let normalized = trimmed.replace(['+', '-'], ",");

        let string_times: Vec<&str> = normalized.split(',').collect();

        for string_time in string_times {
            let string_time = string_time.trim();

            if let Ok(time) = NaiveTime::from_str(string_time) {
                times.push(time);
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
}

#[cfg(test)]
mod time_parser_should {

    use std::str::FromStr;

    use chrono::NaiveTime;
    use rstest::rstest;

    use crate::models::duplicate_finder::TimeKeeper;

    #[rstest]
    #[case("", vec![] )]
    #[case("14:00", vec![NaiveTime::from_str("14:00").unwrap()] )]
    #[case("14:00 + 15:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap()] )]
    #[case("14:00 - 15:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap()] )]
    #[case("14:00 + 15:00 + 16:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap(), NaiveTime::from_str("16:00").unwrap()] )]
    #[case("14:00 + 15:00 - 16:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap(), NaiveTime::from_str("16:00").unwrap()] )]
    #[case("14:00 - 15:00 + 16:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap(), NaiveTime::from_str("16:00").unwrap()] )]
    #[case("14:00 - 15:00 - 16:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap(), NaiveTime::from_str("16:00").unwrap()] )]
    fn parse_time(#[case] time_string: String, #[case] actual_parsed_times: Vec<NaiveTime>) {
        // Given
        let time_keeper = TimeKeeper {
            time_calculation_string: time_string,
            ..Default::default()
        };

        // When
        let parsed_times = time_keeper.parse_times();

        // Then
        for index in 0..actual_parsed_times.len() {
            assert_eq!(actual_parsed_times.get(index), parsed_times.get(index));
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
    fn parse_operation(#[case] time_string: String, #[case] actual_parsed_operations: Vec<char>) {
        // Given
        let time_keeper = TimeKeeper {
            time_calculation_string: time_string,
            ..Default::default()
        };

        // When
        let parsed_operations = time_keeper.parse_operations();

        // Then
        for index in 0..actual_parsed_operations.len() {
            assert_eq!(
                actual_parsed_operations.get(index),
                parsed_operations.get(index)
            );
        }
    }
}
