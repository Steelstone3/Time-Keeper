use chrono::NaiveDateTime;

use crate::models::duplicate_finder::TimeKeeper;

impl TimeKeeper {
    pub fn parse_times(&self) -> Vec<NaiveDateTime> {
        let add = '+';
        let subtract = '-';

        let trimmed_time_calculation_string = self.time_calculation_string.trim();

        vec![]
    }

    pub fn parse_operations(&self) -> Vec<char> {
        todo!()
    }

    fn extract_times() {}

    fn convert_to_time() {}
}

#[cfg(test)]
mod time_parser_should {

    use std::str::FromStr;

    use chrono::NaiveDateTime;
    use rstest::rstest;

    use crate::models::duplicate_finder::TimeKeeper;

    #[rstest]
    #[case("14:00 + 15:00", vec![NaiveDateTime::from_str("14:00").unwrap(), NaiveDateTime::from_str("15:00").unwrap()] )]
    fn parse_succesfully(
        #[case] time_string: String,
        #[case] actual_parsed_times: Vec<NaiveDateTime>,
    ) {
        // Given

        let time_keeper = TimeKeeper {
            time_calculation_string: time_string,
            ..Default::default()
        };

        // When
        let parsed_times = time_keeper.parse_times();

        // Then
        assert_eq!(actual_parsed_times.get(0), parsed_times.get(0))
    }
}
