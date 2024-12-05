use crate::{controllers::time_factory::time_factory, models::time_keeper::TimeKeeper};

impl TimeKeeper {
    pub fn calculate_duration(&self) -> String {
        match time_factory(&self.duration.start_time) {
            Ok(start_time) => match time_factory(&self.duration.end_time) {
                Ok(end_time) => {
                    if end_time < start_time {
                        let duration = start_time - end_time;

                        format!(
                            "{} Hour {} Minutes In The Past\n\n{} Hour {} Minutes From Now",
                            duration.num_hours(),
                            duration.num_minutes() - duration.num_hours() * 60,
                            24 - duration.num_hours(),
                            (duration.num_minutes() - duration.num_hours() * 60)
                        )
                    } else {
                        let duration = end_time - start_time;

                        format!(
                            "{} Hour {} Minutes From Now\n\n{} Hour {} Minutes In The Past",
                            duration.num_hours(),
                            duration.num_minutes() - duration.num_hours() * 60,
                            24 - duration.num_hours(),
                            (duration.num_minutes() - duration.num_hours() * 60)
                        )
                    }
                }
                Err(_) => "End Time could not be parsed".to_string(),
            },
            Err(_) => "Start Time could not be parsed".to_string(),
        }
    }
}

#[cfg(test)]
mod duration_calculator_should {
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
    #[case("asdsadasd", "15:00", "Start Time could not be parsed")]
    #[case("14:00", "asdsadasd", "End Time could not be parsed")]
    fn calculate_end_time(
        #[case] start_time: String,
        #[case] end_time: String,
        #[case] expected_duration_result: String,
    ) {
        // Given
        let time_keeper = TimeKeeper {
            duration: Duration {
                start_time,
                end_time,
                duration_result: "".to_string(),
            },
            ..Default::default()
        };

        // When
        let duration_result = time_keeper.calculate_duration();

        // Then
        assert_eq!(expected_duration_result, duration_result);
    }
}
