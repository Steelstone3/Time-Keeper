use crate::commands::messages::Message;
use crate::models::time_keeper::TimeKeeper;
use iced::widget::column;
use iced::{Element, Theme, widget::Scrollable};

impl TimeKeeper {
    pub fn view(&self) -> Element<'_, Message> {
        let contents = Scrollable::new(column!().push(self.calculator_view()));

        column!(contents).into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}
