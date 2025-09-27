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
            Message::DurationStartTimeChanged(start_time) => {
                self.duration.start_time = start_time;
            }
            Message::DurationEndTimeChanged(end_time) => {
                self.duration.end_time = end_time;
            }
            Message::DurationCalculatePressed => {
                self.duration.duration_result = self.calculate_duration();
            }
            Message::TimeConverterHoursChanged(hours) => {
                self.time_converter.hours =
                    hours.trim().parse::<u32>().unwrap_or_default().clamp(0, 24);
            }
            Message::TimeConverterMinutesChanged(minutes) => {
                self.time_converter.minutes = minutes
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default()
                    .clamp(0, 1440);
            }
            Message::TimeConverterSecondsChanged(seconds) => {
                self.time_converter.seconds = seconds
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default()
                    .clamp(0, 86400);
            }
            Message::TimeConverterCalculatePressed => {
                self.time_converter.time_converter_result = self.convert_time();
            }
        }
    }
}
