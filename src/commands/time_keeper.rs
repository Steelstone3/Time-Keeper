use crate::{commands::messages::Message, models::time_keeper::TimeKeeper};

impl TimeKeeper {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TimeParserChanged(time_input) => self.time_calculation_string = time_input,
            Message::CalculateTimeResultPressed => {
                let times = self.parse_times();
                let operations = self.parse_operations();
                self.time_result = self.calculate_time(times, operations);
            }
        }
    }
}
