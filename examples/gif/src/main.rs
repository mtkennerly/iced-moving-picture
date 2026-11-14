use std::path::PathBuf;

use iced::{
    widget::{button, column, container, row, text},
    window, Element, Length, Size, Task,
};
use iced_moving_picture::widget::gif;

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

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<gif::Frames, gif::Error>),
    ToggledPause,
}

#[derive(Default)]
struct App {
    frames: Option<gif::Frames>,
    paused: bool,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/rust-lang-ferris.gif");

        (
            App::default(),
            gif::Frames::load_from_path(path).map(Message::Loaded),
        )
    }

    fn title(&self) -> String {
        "GIF".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(frames) => {
                self.frames = frames.ok();
            }
            Message::ToggledPause => {
                self.paused = !self.paused;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        if let Some(frames) = self.frames.as_ref() {
            column![
                button(text(if self.paused { "Resume" } else { "Pause" }))
                    .on_press(Message::ToggledPause),
                container(gif(frames).set_paused(self.paused))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
            ]
            .spacing(8)
            .padding(4)
            .into()
        } else {
            row![].into()
        }
    }
}
