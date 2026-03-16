use super::tab_identifier::TabIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    MenuBar,
    FileNew,
    SelectedTabChanged(TabIdentifier),
    ViewThemeChanged,
    EndTimeStartTimeChanged(String),
    EndTimeDurationChanged(String),
    EndTimeCalculatePressed,
    DurationStartChanged(String),
    DurationEndChanged(String),
    DurationCalculatePressed,
    TimeConverterDaysChanged(String),
    TimeConverterHoursChanged(String),
    TimeConverterMinutesChanged(String),
    TimeConverterSecondsChanged(String),
    TimeConverterCalculatePressed,
}
