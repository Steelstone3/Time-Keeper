use crate::{commands::messages::Message, models::time_keeper::TimeKeeper};
use iced::widget::{Column, button, column, text, text_input};
use iced_aw::Card;

impl TimeKeeper {
    pub fn time_converter_view(&self) -> Column<'_, Message> {
        let contents = column!()
            .push(text("Hours"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Hours e.g 14", &self.time_converter.hours.to_string())
                    .on_input(Message::TimeConverterHoursChanged),
            )
            .padding(10)
            .spacing(10)
            .push(text("Minutes"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Minutes e.g 59", &self.time_converter.minutes.to_string())
                    .on_input(Message::TimeConverterMinutesChanged),
            )
            .padding(10)
            .spacing(10)
            .push(text("Seconds"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Seconds e.g 59", &self.time_converter.seconds.to_string())
                    .on_input(Message::TimeConverterSecondsChanged),
            )
            .padding(10)
            .spacing(10)
            .push(button("Calculate").on_press(Message::TimeConverterCalculatePressed))
            .padding(10)
            .spacing(10)
            .push(text(&self.time_converter.time_converter_result))
            .spacing(10)
            .padding(10);

        column!()
            .push(Card::new("Time Converter", contents))
            .spacing(10)
            .padding(10)
    }
}
