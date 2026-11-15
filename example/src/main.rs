use std::path::PathBuf;

use iced::{
    widget::{button, column, container, pick_list, row, slider, text},
    window, Element, Length, Size, Task,
};
use iced_moving_picture::widget::moving_picture;

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
    Loaded(Result<moving_picture::Frames, moving_picture::Error>),
    ToggledPause,
    NewFrame(usize),
    Seeked(usize),
    Kind(moving_picture::Kind),
}

struct App {
    gif: Option<moving_picture::Frames>,
    apng: Option<moving_picture::Frames>,
    paused: bool,
    frame_index: usize,
    kind: moving_picture::Kind,
}

impl Default for App {
    fn default() -> Self {
        Self {
            gif: None,
            apng: None,
            paused: false,
            frame_index: 0,
            kind: moving_picture::Kind::Gif,
        }
    }
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let gif = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/rust-lang-ferris.gif");
        let apng = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/3d2.png");

        (
            App::default(),
            Task::batch([
                moving_picture::Frames::load_from_path(gif, moving_picture::Kind::Gif)
                    .map(Message::Loaded),
                moving_picture::Frames::load_from_path(apng, moving_picture::Kind::Apng)
                    .map(Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        "Moving picture".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(frames) => {
                if let Ok(frames) = frames {
                    match frames.kind() {
                        moving_picture::Kind::Apng => {
                            self.apng = Some(frames);
                        }
                        moving_picture::Kind::Gif => {
                            self.gif = Some(frames);
                        }
                    }
                }
            }
            Message::ToggledPause => {
                self.paused = !self.paused;
            }
            Message::NewFrame(index) => {
                self.frame_index = index;
            }
            Message::Seeked(index) => {
                self.frame_index = index;

                let frames = match self.kind {
                    moving_picture::Kind::Apng => self.apng.as_ref(),
                    moving_picture::Kind::Gif => self.gif.as_ref(),
                };

                if let Some(frames) = frames {
                    frames.seek(moving_picture::Position::Frame(index));
                }
            }
            Message::Kind(kind) => {
                self.kind = kind;
                self.frame_index = 0;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let frames = match self.kind {
            moving_picture::Kind::Apng => self.apng.as_ref(),
            moving_picture::Kind::Gif => self.gif.as_ref(),
        };

        if let Some(frames) = frames {
            let total = frames.len();

            let image = moving_picture(frames)
                .set_paused(self.paused)
                .on_new_frame(Message::NewFrame);

            column![
                row![
                    pick_list(
                        vec![moving_picture::Kind::Gif, moving_picture::Kind::Apng],
                        Some(self.kind),
                        Message::Kind
                    ),
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
