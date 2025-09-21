use crate::{
    commands::{messages::Message, tab_identifier::TabIdentifier},
    models::time_keeper::TimeKeeper,
};
use iced::widget::{Column, Scrollable, column};
use iced_aw::{TabBar, TabLabel};

impl TimeKeeper {
    pub fn tab_bar_view(&self) -> Column<'_, Message> {
        match self.application_state.tab {
            TabIdentifier::EndTime => {
                let tab_bar = selected_tab_bar(&TabIdentifier::EndTime);

                let contents = Scrollable::new(column!().push(self.end_time_view()));

                column!(self.menu_view(), tab_bar, contents)
            }
            TabIdentifier::Duration => {
                let tab_bar = selected_tab_bar(&TabIdentifier::Duration);

                let contents = Scrollable::new(column!().push(self.duration_view()));

                column!(self.menu_view(), tab_bar, contents)
            }
        }
    }
}

fn selected_tab_bar(active_tab: &TabIdentifier) -> TabBar<'static, Message, TabIdentifier> {
    TabBar::new(Message::SelectedTabChanged)
        .push(
            TabIdentifier::EndTime,
            TabLabel::IconText('\u{1F551}', "End Time".to_string()),
        )
        .push(
            TabIdentifier::Duration,
            TabLabel::IconText('\u{231B}', "Duration".to_string()),
        )
        .set_active_tab(active_tab)
}
