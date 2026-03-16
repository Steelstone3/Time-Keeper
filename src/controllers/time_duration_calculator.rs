use chrono::ParseError;

use crate::{controllers::time_factory::time_factory, models::time_keeper::TimeKeeper};

impl TimeKeeper {
    pub fn calculate_time_duration(&self) -> Result<String, ParseError> {
        match time_factory(&self.duration.start) {
            Ok(start_time) => match time_factory(&self.duration.end) {
                Ok(end_time) => {
                    if end_time < start_time {
                        let duration = start_time - end_time;

                        Ok(format!(
                            "{} Hour {} Minutes In The Past\n\n{} Hour {} Minutes From Now",
                            duration.num_hours(),
                            duration.num_minutes() - duration.num_hours() * 60,
                            24 - duration.num_hours(),
                            (duration.num_minutes() - duration.num_hours() * 60)
                        ))
                    } else {
                        let duration = end_time - start_time;

                        Ok(format!(
                            "{} Hour {} Minutes From Now\n\n{} Hour {} Minutes In The Past",
                            duration.num_hours(),
                            duration.num_minutes() - duration.num_hours() * 60,
                            24 - duration.num_hours(),
                            (duration.num_minutes() - duration.num_hours() * 60)
                        ))
                    }
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod time_duration_calculator_should {
    use crate::models::{duration::Duration, time_keeper::TimeKeeper};
    use rstest::rstest;

    #[rstest]
    #[case(
        "14:00",
        "15:00",
        "1 Hour 0 Minutes From Now\n\n23 Hour 0 Minutes In The Past"
    )]
    #[case(
        "13:00",
        "16:00",
        "3 Hour 0 Minutes From Now\n\n21 Hour 0 Minutes In The Past"
    )]
    #[case(
        "16:00",
        "13:00",
        "3 Hour 0 Minutes In The Past\n\n21 Hour 0 Minutes From Now"
    )]
    #[case(
        "14:00",
        "13:00",
        "1 Hour 0 Minutes In The Past\n\n23 Hour 0 Minutes From Now"
    )]
    #[case(
        "14:00",
        "13:59",
        "0 Hour 1 Minutes In The Past\n\n24 Hour 1 Minutes From Now"
    )]
    #[case(
        "13:00",
        "13:00",
        "0 Hour 0 Minutes From Now\n\n24 Hour 0 Minutes In The Past"
    )]
    fn calculate_end_time(
        #[case] start: String,
        #[case] end: String,
        #[case] expected_duration_result: String,
    ) {
        // Given
        let time_keeper = TimeKeeper {
            duration: Duration {
                start,
                end,
                duration_result: "".to_string(),
            },
            ..Default::default()
        };

        // When
        let duration_result = time_keeper.calculate_time_duration();

        // Then
        assert_eq!(expected_duration_result, duration_result.unwrap());
    }
}
