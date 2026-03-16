use crate::models::time_keeper::TimeKeeper;
use chrono::Duration;

impl TimeKeeper {
    pub fn convert_time(&self) -> String {
        let duration = Duration::days(self.time_converter.days as i64)
            + Duration::hours(self.time_converter.hours as i64)
            + Duration::minutes(self.time_converter.minutes as i64)
            + Duration::seconds(self.time_converter.seconds as i64);

        let days = duration.num_days();
        let hours = duration.num_hours();
        let minutes = (duration.num_minutes() % 60).abs();
        let seconds = (duration.num_seconds() % 60).abs();

        if days == 0 && hours < 24 {
            format!(
                "{:01} Days\n\n{:02}:{:02}:{:02} Hours\n\n{:02}:{:02} Minutes\n\n{} Seconds",
                days,
                hours,
                minutes,
                seconds,
                duration.num_minutes(),
                seconds,
                duration.num_seconds()
            )
        } else if days == 1 && hours % 24 == 0 && minutes == 0 && seconds == 0 {
            format!(
                "{:01} Day\n\n{:02}:{:02}:{:02} Hours\n\n{:02}:{:02} Minutes\n\n{} Seconds",
                days,
                hours,
                minutes,
                seconds,
                duration.num_minutes(),
                seconds,
                duration.num_seconds()
            )
        } else if days == 1 {
            format!(
                "~{:01} Day\n\n{:02}:{:02}:{:02} Hours\n\n{:02}:{:02} Minutes\n\n{} Seconds",
                days,
                hours,
                minutes,
                seconds,
                duration.num_minutes(),
                seconds,
                duration.num_seconds()
            )
        } else if hours % 24 == 0 {
            format!(
                "{:01} Days\n\n{:02}:{:02}:{:02} Hours\n\n{:02}:{:02} Minutes\n\n{} Seconds",
                days,
                hours,
                minutes,
                seconds,
                duration.num_minutes(),
                seconds,
                duration.num_seconds()
            )
        } else {
            format!(
                "~{:01} Days\n\n{:02}:{:02}:{:02} Hours\n\n{:02}:{:02} Minutes\n\n{} Seconds",
                days,
                hours,
                minutes,
                seconds,
                duration.num_minutes(),
                seconds,
                duration.num_seconds()
            )
        }
    }
}

#[cfg(test)]
mod time_converter_should {
    use crate::models::{time_converter::TimeConverter, time_keeper::TimeKeeper};
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 0, 0, "0 Days\n\n00:00:00 Hours\n\n00:00 Minutes\n\n0 Seconds")]
    #[case(0, 0, 0, 1, "0 Days\n\n00:00:01 Hours\n\n00:01 Minutes\n\n1 Seconds")]
    #[case(0, 0, 0, 59, "0 Days\n\n00:00:59 Hours\n\n00:59 Minutes\n\n59 Seconds")]
    #[case(0, 0, 0, 61, "0 Days\n\n00:01:01 Hours\n\n01:01 Minutes\n\n61 Seconds")]
    #[case(0, 0, 1, 0, "0 Days\n\n00:01:00 Hours\n\n01:00 Minutes\n\n60 Seconds")]
    #[case(
        0,
        0,
        59,
        0,
        "0 Days\n\n00:59:00 Hours\n\n59:00 Minutes\n\n3540 Seconds"
    )]
    #[case(
        0,
        0,
        61,
        0,
        "0 Days\n\n01:01:00 Hours\n\n61:00 Minutes\n\n3660 Seconds"
    )]
    #[case(
        0,
        1,
        0,
        0,
        "0 Days\n\n01:00:00 Hours\n\n60:00 Minutes\n\n3600 Seconds"
    )]
    #[case(
        0,
        24,
        0,
        0,
        "1 Day\n\n24:00:00 Hours\n\n1440:00 Minutes\n\n86400 Seconds"
    )]
    #[case(
        1,
        0,
        0,
        0,
        "1 Day\n\n24:00:00 Hours\n\n1440:00 Minutes\n\n86400 Seconds"
    )]
    #[case(
        7,
        0,
        0,
        0,
        "7 Days\n\n168:00:00 Hours\n\n10080:00 Minutes\n\n604800 Seconds"
    )]
    #[case(
        1,
        1,
        1,
        1,
        "~1 Day\n\n25:01:01 Hours\n\n1501:01 Minutes\n\n90061 Seconds"
    )]
    fn calculate_end_time(
        #[case] days: u32,
        #[case] hours: u32,
        #[case] minutes: u32,
        #[case] seconds: u32,
        #[case] expected_time_converter_result: String,
    ) {
        // Given
        let time_keeper = TimeKeeper {
            time_converter: TimeConverter {
                days,
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
        pretty_assertions::assert_eq!(expected_time_converter_result, time_converter_result);
    }
}
