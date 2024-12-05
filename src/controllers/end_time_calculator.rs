use crate::{
    controllers::{duration_factory::duration_factory, time_factory::time_factory},
    models::time_keeper::TimeKeeper,
};

impl TimeKeeper {
    pub fn calculate_end_time(&self) -> String {
        match time_factory(&self.end_time.start_time) {
            Ok(time) => match duration_factory(&self.end_time.duration) {
                Ok(duration) => {
                    let end_time = time + duration;

                    format!(
                        "{} Hours {} Minutes From {} Is {}",
                        duration.num_hours(),
                        duration.num_minutes() - duration.num_hours() * 60,
                        self.end_time.start_time,
                        end_time
                    )
                }
                Err(_) => "Duration could not be parsed".to_string(),
            },
            Err(_) => "Start Time could not be parsed".to_string(),
        }
    }
}

#[cfg(test)]
mod end_time_calculator_should {
    use crate::models::{end_time::EndTime, time_keeper::TimeKeeper};
    use rstest::rstest;

    #[rstest]
    #[case("14:00", "6", "6 Hours 0 Minutes From 14:00 Is 20:00:00")]
    #[case("14:00", "3", "3 Hours 0 Minutes From 14:00 Is 17:00:00")]
    #[case("14:00", "3.5", "3 Hours 30 Minutes From 14:00 Is 17:30:00")]
    #[case("14:00", "6.5", "6 Hours 30 Minutes From 14:00 Is 20:30:00")]
    #[case("14:00", "1:50", "1 Hours 50 Minutes From 14:00 Is 15:50:00")]
    #[case("14:00", "5:30", "5 Hours 30 Minutes From 14:00 Is 19:30:00")]
    #[case("asdasdsad", "3", "Start Time could not be parsed")]
    #[case("14:00", "asdasdasd", "Duration could not be parsed")]
    #[case("asdasdsad", "asdasdasd", "Start Time could not be parsed")]
    fn calculate_end_time(
        #[case] start_time: String,
        #[case] duration: String,
        #[case] expected_end_time_result: String,
    ) {
        // Given
        let time_keeper = TimeKeeper {
            end_time: EndTime {
                start_time,
                duration,
                end_time_result: "".to_string(),
            },
            ..Default::default()
        };

        // When
        let end_time_result = time_keeper.calculate_end_time();

        // Then
        assert_eq!(expected_end_time_result, end_time_result);
    }
}
