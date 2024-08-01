use crate::app::{Application, Message};

pub(crate) fn reduce(this: &mut Application, message: Message) -> iced::Command<Message>  {
    match message {
        Message::InputPath(path) => {
            this.input_path = path;
            iced::Command::none()
        },
        Message::SelectPath => {
            rfd::FileDialog::new()
                .pick_folder()
                .map(|path| {
                    this.input_path = path.to_string_lossy().to_string();
                })
                .unwrap_or_default();
            iced::Command::none()
        },
        Message::OutputPath(path) => {
            this.output_path = path;
            iced::Command::none()
        },
        Message::SelectOutputPath => {
            rfd::FileDialog::new()
                .pick_folder()
                .map(|path| {
                    this.output_path = path.to_string_lossy().to_string();
                })
                .unwrap_or_default();
            iced::Command::none()
        },
        Message::Start => {
            this.progress = 0.0;
            this.state = crate::app::State::Running;
            iced::Command::none()
        },
        Message::End => {
            this.state = crate::app::State::Idle;
            iced::Command::none()
        },
        Message::Progress(progress) => {
            this.progress = progress;
            iced::Command::none()
        }
    }
}