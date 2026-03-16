use crate::{commands::messages::Message, models::time_keeper::TimeKeeper};
use iced::widget::{Column, button, column, text, text_input};
use iced_aw::Card;

impl TimeKeeper {
    pub fn duration_view(&self) -> Column<'_, Message> {
        let contents = column!()
            .push(text("Start Date/Time"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("e.g 12/5/2025 or 14:00", &self.duration.start)
                    .on_input(Message::DurationStartChanged),
            )
            .padding(10)
            .spacing(10)
            .push(text("End Date/Time"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("e.g 16/5/2025 or 13:00", &self.duration.end)
                    .on_input(Message::DurationEndChanged),
            )
            .padding(10)
            .spacing(10)
            .push(button("Calculate").on_press(Message::DurationCalculatePressed))
            .padding(10)
            .spacing(10)
            .push(text(&self.duration.duration_result))
            .spacing(10)
            .padding(10);

        column!()
            .push(Card::new("Duration Calculator", contents))
            .spacing(10)
            .padding(10)
    }
}
