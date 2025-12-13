use std::path::PathBuf;

use iced::{
    widget::{container, row},
    window, Element, Length, Size, Task,
};
use iced_moving_picture::widget::apng;

fn main() {
    iced::application(App::new, App::update, App::view)
        .window(window::Settings {
            size: Size::new(498.0, 164.0),
            ..Default::default()
        })
        .title(App::title)
        .run()
        .unwrap()
}

#[derive(Debug)]
enum Message {
    Loaded(Result<apng::Frames, apng::Error>),
}

#[derive(Default)]
struct App {
    frames: Option<apng::Frames>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/3d2.png");

        (
            App::default(),
            apng::Frames::load_from_path(path).map(Message::Loaded),
        )
    }

    fn title(&self) -> String {
        "APNG".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let Message::Loaded(frames) = message;

        self.frames = frames.ok();

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        if let Some(frames) = self.frames.as_ref() {
            container(apng(frames))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            row![].into()
        }
    }
}
