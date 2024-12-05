use crate::models::duplicate_finder::DuplicateFinder;
use iced::widget::text_editor::{Action, Edit};

impl DuplicateFinder {
    pub fn get_editor_text(&self) -> String {
        self.application_state.content.text()
    }

    pub fn refresh_editor(&mut self, content: String) {
        self.application_state.content.perform(Action::SelectAll);

        self.application_state
            .content
            .perform(Action::Edit(Edit::Paste(content.into())));
    }
}
