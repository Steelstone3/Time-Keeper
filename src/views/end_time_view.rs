use crate::{commands::messages::Message, models::time_keeper::TimeKeeper};
use iced::widget::{Column, button, column, text, text_input};
use iced_aw::Card;

impl TimeKeeper {
    pub fn end_time_view(&self) -> Column<'_, Message> {
        let contents = column!()
            .push(text("Start Time"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("e.g 14:00", &self.end_time.start_time)
                    .on_input(Message::EndTimeStartTimeChanged),
            )
            .padding(10)
            .spacing(10)
            .push(text("Duration"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("e.g 6.5 or 1:30", &self.end_time.duration)
                    .on_input(Message::EndTimeDurationChanged),
            )
            .padding(10)
            .spacing(10)
            .push(button("Calculate").on_press(Message::EndTimeCalculatePressed))
            .padding(10)
            .spacing(10)
            .push(text(&self.end_time.end_time_result))
            .spacing(10)
            .padding(10);

        column!()
            .push(Card::new("End Time Calculator", contents))
            .spacing(10)
            .padding(10)
    }
}
