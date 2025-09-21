use crate::{commands::messages::Message, models::duplicate_finder::TimeKeeper};

impl TimeKeeper {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TimeParserChanged(time_input) => self.time_calculation_string = time_input,
            Message::CalculateTimeResultPressed => {
                self.parse_times();
                self.parse_operations();
                self.time_result = self.calculate_time();
            }
        }
    }
}
