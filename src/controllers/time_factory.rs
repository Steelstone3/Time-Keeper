use std::str::FromStr;

use chrono::NaiveTime;

// TODO AH test
pub fn time_factory(time: &str) -> Result<NaiveTime, chrono::ParseError> {
    NaiveTime::from_str(time)
}

#[cfg(test)]
mod time_factory_should {}
