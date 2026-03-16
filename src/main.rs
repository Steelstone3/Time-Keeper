use iced::Settings;
use models::time_keeper::TimeKeeper;

mod commands;
mod controllers;
mod models;
mod views;

pub fn main() -> iced::Result {
    iced::application(TimeKeeper::boot, TimeKeeper::update, TimeKeeper::view)
        .theme(TimeKeeper::theme)
        .antialiasing(true)
        .settings(Settings {
            id: Some("Time Keeper".to_string()),
            ..Default::default()
        })
        .run()
}
