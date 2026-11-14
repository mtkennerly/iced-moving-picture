use std::path::PathBuf;

use iced::{
    widget::{button, column, container, row, slider, text},
    window, Element, Length, Size, Task,
};
use iced_moving_picture::{widget::apng, Position};

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
    Loaded(Result<apng::Frames, apng::Error>),
    ToggledPause,
    NewFrame(usize),
    Seeked(usize),
}

#[derive(Default)]
struct App {
    frames: Option<apng::Frames>,
    paused: bool,
    frame_index: usize,
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
        match message {
            Message::Loaded(frames) => {
                self.frames = frames.ok();
            }
            Message::ToggledPause => {
                self.paused = !self.paused;
            }
            Message::NewFrame(index) => {
                self.frame_index = index;
            }
            Message::Seeked(index) => {
                self.frame_index = index;
                if let Some(frames) = &self.frames {
                    frames.seek(Position::Frame(index));
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        if let Some(frames) = self.frames.as_ref() {
            let total = frames.len();

            let image = apng(frames)
                .set_paused(self.paused)
                .on_new_frame(Message::NewFrame);

            column![
                row![
                    button(text(if self.paused { "Resume" } else { "Pause" }))
                        .on_press(Message::ToggledPause),
                    text(format!("frame: {}/{}", self.frame_index, total)),
                    text(format!(
                        "time: {:.2}/{:.2}",
                        frames.duration_at(self.frame_index).as_secs_f64(),
                        frames.duration().as_secs_f64()
                    )),
                ]
                .spacing(8),
                slider(0.0..=(total as f64), self.frame_index as f64, |v| {
                    Message::Seeked(v as usize)
                })
                .width(Length::Fill),
                container(image)
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
