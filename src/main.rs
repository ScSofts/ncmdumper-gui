use iced::{Application as App, Settings, Size};
use app::Application as Application;
mod app;

#[tokio::main]
async fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = Size::new(600.0, 480.0);
    settings.window.resizable = false;
    Application::run(settings)
}