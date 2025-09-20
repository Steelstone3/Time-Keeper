use crate::{commands::messages::Message, models::duplicate_finder::TimeKeeper};
use iced::widget::{Column, button, column, text, text_editor};
use iced_aw::Card;

impl TimeKeeper {
    pub fn calculator_view(&self) -> Column<'_, Message> {
        column!()
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
