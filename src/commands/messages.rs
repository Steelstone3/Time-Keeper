use iced::widget::text_editor;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    TimeParserChanged(String),
    CalculateTimeResultPressed,
}
