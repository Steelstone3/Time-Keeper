use crate::commands::tab_identifier::TabIdentifier;

#[derive(Default)]
pub struct ApplicationState {
    pub is_light_theme: bool,
    pub tab: TabIdentifier,
}
