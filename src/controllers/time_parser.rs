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
        let add = '+';
        let subtract = '-';

        let trimmed_time_calculation_string = self.time_calculation_string.trim();

        vec![]
    }

    fn extract_times() {}

    fn convert_to_time() {}
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
    #[case("14:00 + 15:00 + 16:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap(), NaiveTime::from_str("16:00").unwrap()] )]
    #[case("14:00 + 15:00 + 16:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap(), NaiveTime::from_str("16:00").unwrap()] )]
    #[case("14:00 + 15:00 - 16:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap(), NaiveTime::from_str("16:00").unwrap()] )]
    #[case("14:00 - 15:00 + 16:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap(), NaiveTime::from_str("16:00").unwrap()] )]
    #[case("14:00 - 15:00 - 16:00", vec![NaiveTime::from_str("14:00").unwrap(), NaiveTime::from_str("15:00").unwrap(), NaiveTime::from_str("16:00").unwrap()] )]
    fn parse_succesfully(#[case] time_string: String, #[case] actual_parsed_times: Vec<NaiveTime>) {
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
}
