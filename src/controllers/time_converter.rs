use crate::models::time_keeper::TimeKeeper;
use chrono::Duration;

impl TimeKeeper {
    pub fn convert_time(&self) -> String {
        let duration = Duration::hours(self.time_converter.hours as i64)
            + Duration::minutes(self.time_converter.minutes as i64)
            + Duration::seconds(self.time_converter.seconds as i64);

        let hours = duration.num_hours();
        let minutes = (duration.num_minutes() % 60).abs();
        let seconds = (duration.num_seconds() % 60).abs();

        format!(
            "{:02}:{:02}:{:02} Hours\n\n{:02}:{:02} Minutes\n\n{} Seconds",
            hours,
            minutes,
            seconds,
            duration.num_minutes(),
            seconds,
            duration.num_seconds()
        )
    }
}

#[cfg(test)]
mod end_time_calculator_should {
    use crate::models::{time_converter::TimeConverter, time_keeper::TimeKeeper};
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 0, "00:00:00 Hours\n\n00:00 Minutes\n\n0 Seconds")]
    #[case(0, 0, 1, "00:00:01 Hours\n\n00:01 Minutes\n\n1 Seconds")]
    #[case(0, 0, 59, "00:00:59 Hours\n\n00:59 Minutes\n\n59 Seconds")]
    #[case(0, 0, 61, "00:01:01 Hours\n\n01:01 Minutes\n\n61 Seconds")]
    #[case(0, 1, 0, "00:01:00 Hours\n\n01:00 Minutes\n\n60 Seconds")]
    #[case(0, 59, 0, "00:59:00 Hours\n\n59:00 Minutes\n\n3540 Seconds")]
    #[case(0, 61, 0, "01:01:00 Hours\n\n61:00 Minutes\n\n3660 Seconds")]
    #[case(1, 0, 0, "01:00:00 Hours\n\n60:00 Minutes\n\n3600 Seconds")]
    #[case(24, 0, 0, "24:00:00 Hours\n\n1440:00 Minutes\n\n86400 Seconds")]
    fn calculate_end_time(
        #[case] hours: u32,
        #[case] minutes: u32,
        #[case] seconds: u32,
        #[case] expected_time_converter_result: String,
    ) {
        // Given
        let time_keeper = TimeKeeper {
            time_converter: TimeConverter {
                hours,
                minutes,
                seconds,
                time_converter_result: "".to_string(),
            },
            ..Default::default()
        };

        // When
        let time_converter_result = time_keeper.convert_time();

        // Then
        assert_eq!(expected_time_converter_result, time_converter_result);
    }
}
