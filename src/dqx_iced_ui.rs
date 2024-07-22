use iced::{
    executor,
    widget::{button, column, row, text},
    Application, Command,
};

use crate::dqx_text_model::DqxTranslationsModel;

#[derive(Default, Debug)]
pub struct DqxTextApp {
    data: Option<DqxTranslationsModel>,
}

#[derive(Debug, Clone)]
pub enum DqxTextAppMessage {
    DoOpen,
    DoSave,
}

impl Application for DqxTextApp {
    type Executor = executor::Default;
    type Message = DqxTextAppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { data: None }, Command::none())
    }

    fn title(&self) -> String {
        String::from("DQX Translation Helper")
    }

    fn update(&mut self, message: DqxTextAppMessage) -> iced::Command<Self::Message> {
        match message {
            DqxTextAppMessage::DoOpen => {
                println!("Opening a file...");
                Command::none()
            }
            DqxTextAppMessage::DoSave => {
                println!("Saving the current file...");
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let header = row![
            button("Open").on_press(DqxTextAppMessage::DoOpen),
            button("Save").on_press(DqxTextAppMessage::DoSave)
        ];

        let body = column![text("Hello world")];

        column![header, body].into()
    }
}
