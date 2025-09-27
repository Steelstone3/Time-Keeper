use crate::models::{duration::Duration, end_time::EndTime, time_converter::TimeConverter};

use super::application_state::ApplicationState;

#[derive(Default)]
pub struct TimeKeeper {
    pub application_state: ApplicationState,
    pub end_time: EndTime,
    pub duration: Duration,
    pub time_converter: TimeConverter,
}
