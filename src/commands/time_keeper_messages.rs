use crate::{commands::messages::Message, models::time_keeper::TimeKeeper};

impl TimeKeeper {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::MenuBar => {}
            Message::SelectedTabChanged(tab_identifier) => self.switch_tab(tab_identifier),
            Message::FileNew => *self = TimeKeeper::default(),
            Message::ViewThemeChanged => self.switch_theme(),
            Message::EndTimeStartTimeChanged(start_time) => {
                self.end_time.start_time = start_time;
            }
            Message::EndTimeDurationChanged(duration) => {
                self.end_time.duration = duration;
            }
            Message::EndTimeCalculatePressed => {
                self.end_time.end_time_result = self.calculate_end_time();
            }
            Message::DurationStartChanged(start) => {
                self.duration.start = start;
            }
            Message::DurationEndChanged(end) => {
                self.duration.end = end;
            }
            Message::DurationCalculatePressed => {
                self.duration.duration_result = self
                    .calculate_time_duration()
                    .or_else(|_| self.calculate_date_duration())
                    .unwrap_or_else(|_| "Failed to parse".to_string());
            }
            Message::TimeConverterDaysChanged(days) => {
                self.time_converter.days = days
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default()
                    .clamp(0, u32::MAX / 86400); // Seconds to days
            }
            Message::TimeConverterHoursChanged(hours) => {
                self.time_converter.hours = hours
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default()
                    .clamp(0, u32::MAX / 3600) // Seconds to hours;
            }
            Message::TimeConverterMinutesChanged(minutes) => {
                self.time_converter.minutes = minutes
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default()
                    .clamp(0, u32::MAX / 60); // Seconds to minutes
            }
            Message::TimeConverterSecondsChanged(seconds) => {
                self.time_converter.seconds = seconds
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default()
                    .clamp(0, u32::MAX);
            }
            Message::TimeConverterCalculatePressed => {
                self.time_converter.time_converter_result = self.convert_time();
            }
        }
    }
}
