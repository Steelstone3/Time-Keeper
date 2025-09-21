use iced::Settings;
use models::time_keeper::TimeKeeper;

mod commands;
mod controllers;
mod models;
mod views;

pub fn main() -> iced::Result {
    iced::application("Time Keeper", TimeKeeper::update, TimeKeeper::view)
        .theme(TimeKeeper::theme)
        .antialiasing(true)
        .settings(Settings::default())
        .run()
}
