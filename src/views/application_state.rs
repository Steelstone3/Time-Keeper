use crate::{
    commands::{messages::Message, tab_identifier::TabIdentifier},
    models::time_keeper::TimeKeeper,
};
use iced::{Element, Theme};

impl TimeKeeper {
    pub fn view(&self) -> Element<'_, Message> {
        self.tab_bar_view().into()
    }

    pub fn switch_tab(&mut self, tab_identifier: TabIdentifier) {
        match tab_identifier {
            TabIdentifier::EndTime => self.application_state.tab = TabIdentifier::EndTime,
            TabIdentifier::Duration => self.application_state.tab = TabIdentifier::Duration,
            TabIdentifier::TimeConverter => {
                self.application_state.tab = TabIdentifier::TimeConverter
            }
        }
    }

    pub fn theme(&self) -> Theme {
        match self.application_state.is_light_theme {
            true => Theme::Light,
            false => Theme::Dark,
        }
    }

    pub fn switch_theme(&mut self) {
        match self.application_state.is_light_theme {
            true => self.application_state.is_light_theme = false,
            false => self.application_state.is_light_theme = true,
        }
    }
}
