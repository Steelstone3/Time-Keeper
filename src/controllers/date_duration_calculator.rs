use crate::{controllers::date_factory::date_factory, models::time_keeper::TimeKeeper};

impl TimeKeeper {
    pub fn calculate_date_duration(&self) -> Result<String, ()> {
        match date_factory(&self.duration.start) {
            Ok(start_date) => match date_factory(&self.duration.end) {
                Ok(end_date) => {
                    if (end_date - start_date).num_days() < 0 {
                        Ok((start_date - end_date).num_days().to_string() + " Days")
                    } else {
                        Ok((end_date - start_date).num_days().to_string() + " Days")
                    }
                }
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod date_duration_calculator_should {
    use crate::models::{duration::Duration, time_keeper::TimeKeeper};
    use rstest::rstest;

    #[rstest]
    #[case("11/11/2026", "11/11/2026", "0 Days")]
    #[case("16/11/2026", "11/11/2026", "5 Days")]
    #[case("11/11/2026", "16/11/2026", "5 Days")]
    #[case("11/11/2026", "11/12/2026", "30 Days")]
    #[case("31/1/2026", "28/2/2026", "28 Days")]
    #[case("31/1/2028", "29/2/2028", "29 Days")]
    #[case("1/1/2026", "1/1/2027", "365 Days")]
    #[case("1/1/2028", "1/1/2029", "366 Days")]
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
        let duration_result = time_keeper.calculate_date_duration();

        // Then
        pretty_assertions::assert_eq!(expected_duration_result, duration_result.unwrap());
    }
}
