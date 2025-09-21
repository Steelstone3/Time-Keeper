use crate::models::{time_keeper::TimeKeeper, time_or_duration::TimeOrDuration};
use chrono::NaiveTime;

impl TimeKeeper {
    pub fn calculate_time(&self, times: Vec<TimeOrDuration>, operations: Vec<char>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod time_calculator_should {
    use crate::models::{time_keeper::TimeKeeper, time_or_duration::TimeOrDuration};
    use chrono::NaiveTime;
    use rstest::rstest;
    use std::str::FromStr;

    // #[rstest]
    // #[case("14:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap())], vec![], "14:00" )]
    // #[case("14:00 - 13:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("13:00").unwrap())], vec!['-'], "1 Hour From Now" )]
    // #[case("13:00 - 14:00", vec![TimeOrDuration::new_time(NaiveTime::from_str("13:00").unwrap()), TimeOrDuration::new_time(NaiveTime::from_str("14:00").unwrap())], vec!['-'], "1 Hour In The Past" )]
    // #[case("13:00 + 6", vec![TimeOrDuration::new_time(NaiveTime::from_str("13:00").unwrap())], vec!['+'], "1 Hour In The Past" )]
    // fn parse_time(
    //     #[case] time_string: String,
    //     #[case] times: Vec<TimeOrDuration>,
    //     #[case] operations: Vec<char>,
    //     #[case] expected_result_string: String,
    // ) {
    //     // Given
    //     let time_keeper = TimeKeeper {
    //         time_calculation_string: time_string,
    //         ..Default::default()
    //     };

    //     // When
    //     let result = time_keeper.calculate_time(times, operations);

    //     // Then
    //     assert_eq!(expected_result_string, result)
    // }
}
