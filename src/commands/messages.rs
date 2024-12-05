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
    DurationStartTimeChanged(String),
    DurationEndTimeChanged(String),
    DurationCalculatePressed,
}
