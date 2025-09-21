use std::str::FromStr;

use chrono::NaiveTime;

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
    #[case("14:00", NaiveTime::from_str("14:00").unwrap())]
    #[case("15:00", NaiveTime::from_str("15:00").unwrap())]
    #[case("15:23", NaiveTime::from_str("15:23").unwrap())]
    #[case("15:23:12", NaiveTime::from_str("15:23:12").unwrap())]
    fn create(#[case] time: String, #[case] expected_time_result: NaiveTime) {
        // When
        let time_result = time_factory(&time);

        // Then
        assert_eq!(expected_time_result, time_result.unwrap())
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
        // When
        let _ = time_factory(&time).unwrap();
    }
}
