use chrono::NaiveTime;
use std::str::FromStr;

pub fn time_factory(time: &str) -> Result<NaiveTime, chrono::ParseError> {
    NaiveTime::from_str(time)
}

#[cfg(test)]
mod time_factory_should {
    use crate::controllers::time_factory::time_factory;
    use chrono::NaiveTime;
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case("14:00", NaiveTime::from_str("14:00").unwrap_or_default())]
    #[case("15:00", NaiveTime::from_str("15:00").unwrap_or_default())]
    #[case("15:23", NaiveTime::from_str("15:23").unwrap_or_default())]
    #[case("15:23:12", NaiveTime::from_str("15:23:12").unwrap_or_default())]
    fn create(#[case] time: String, #[case] expected_time_result: NaiveTime) {
        // When
        let time_result = time_factory(&time);

        // Then
        assert_eq!(Ok(expected_time_result), time_result)
    }

    #[rstest]
    #[case("asdsad")]
    #[case("12:xx")]
    #[case("xx:12")]
    #[case("::")]
    #[case("-23")]
    #[case("-23:00")]
    #[case("23:60")]
    #[case("24")]
    #[case("24:00")]
    #[should_panic]
    fn create_invalid(#[case] time: String) {
        // Given
        let expected_time = NaiveTime::from_str("00:00").unwrap_or_default();

        // When
        let time = time_factory(&time);

        // Then
        pretty_assertions::assert_eq!(Some(expected_time), time.ok());
    }
}
