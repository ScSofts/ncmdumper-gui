use iced::{Element, Subscription, Theme};

mod view;
mod reducer;

mod res;
mod style;
mod convert;
mod metadata;

#[derive(Default, Debug)]
pub struct Application{
    input_path: String,
    output_path: String,
    progress: f32,
    state: State
}

#[derive(Debug, Clone)]
pub enum Message{
    InputPath(String),
    SelectPath,
    OutputPath(String),
    SelectOutputPath,
    Start,
    End,
    Progress(f32)
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum State{
    Running,
    #[default]
    Idle
}

impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self{
                ..Default::default()
            },
            iced::Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("NCM Dumper GUI")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        reducer::reduce(self, message)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        view::view(self)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let process: Subscription<Message> = match self.state {
            State::Running => {
                convert::start(0, self.input_path.clone(), self.output_path.clone())
                    .map(|(_id, progress, converting)| {
                        if converting{
                            Message::Progress(progress)
                        }else{
                            Message::End
                        }
                    })
            },
            _ => Subscription::none()
        };
        Subscription::batch(vec![
            process
        ])
    }
}
