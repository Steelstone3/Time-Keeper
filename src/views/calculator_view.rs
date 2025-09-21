use crate::{commands::messages::Message, models::duplicate_finder::TimeKeeper};
use iced::widget::{Column, button, column, text, text_input};


impl TimeKeeper {
    pub fn calculator_view(&self) -> Column<'_, Message> {
        let contents = column!()
            .push(text("Time Parser"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("e.g 5pm - 14:34", &self.time_calculation_string)
                    .on_input(Message::TimeParserChanged),
            )
            .spacing(10)
            .padding(10)
            .push(button("Calculate").on_press(Message::CalculateTimeResultPressed))
            .push(text(&self.time_result))
            .spacing(10)
            .padding(10);

        contents
        // let contents = column!()
        //     .push(text("Duplicate Text"))
        //     .spacing(10)
        //     .push(button("Refresh Duplicate Search").on_press(Message::DuplicateSearchPressed))
        //     .padding(10)
        //     .spacing(10)
        //     .push(
        //         text_editor(&self.content)
        //             .placeholder("Type something here...")
        //             .on_action(Message::EditorChanged),
        //     )
        //     .spacing(10)
        //     .padding(10);

        // column!()
        //     .push(Card::new("Editor", contents))
        //     .spacing(10)
        //     .padding(10)
    }
}
