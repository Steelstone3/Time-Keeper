use chrono::NaiveDate;

pub fn date_factory(date: &str) -> Result<NaiveDate, ()> {
    let dates: Vec<&str> = date.split('/').collect();

    if dates.len() != 3 {
        return Err(());
    }

    let year = dates[2].parse::<i32>().unwrap_or(2000);
    let month = dates[1].parse::<u32>().unwrap_or(1);
    let day = dates[0].parse::<u32>().unwrap_or(1);

    NaiveDate::from_ymd_opt(year, month, day)
        .or_else(|| NaiveDate::from_ymd_opt(2000, 1, 1))
        .ok_or(())
}

#[cfg(test)]
mod date_factory_should {
    use crate::controllers::date_factory::date_factory;
    use chrono::NaiveDate;
    use rstest::rstest;

    #[rstest]
    #[case("11/11/2026", NaiveDate::from_ymd_opt(2026, 11, 11).unwrap())]
    #[case("31/1/2025", NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())]
    #[case("29/2/2028", NaiveDate::from_ymd_opt(2028, 2, 29).unwrap())]
    #[case("28/2/2026", NaiveDate::from_ymd_opt(2026, 2, 28).unwrap())]
    #[case("29/2/2026", NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())]
    #[case("32/1/2025", NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())]
    #[case("//",  NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())]
    #[case("x/x/x",  NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())]
    fn create(#[case] time: String, #[case] expected_date_result: NaiveDate) {
        // When
        let date_result = date_factory(&time);

        // Then
        pretty_assertions::assert_eq!(expected_date_result, date_result.unwrap())
    }

    #[rstest]
    #[case("asdsad")]
    #[case("/")]
    #[case("x/x")]
    #[case("23")]
    #[case("23/2")]
    #[should_panic]
    fn create_invalid(#[case] date: String) {
        // When
        let _ = date_factory(&date).unwrap();
    }
}
