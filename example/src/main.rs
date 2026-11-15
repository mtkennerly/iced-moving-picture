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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Speed(f64);

impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x", self.0)
    }
}

impl From<Speed> for f64 {
    fn from(speed: Speed) -> Self {
        speed.0
    }
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<moving_picture::Frames, moving_picture::Error>),
    ToggledPause,
    ToggledLoop,
    SetSpeed(Speed),
    NewFrame(usize),
    LastFrame,
    Seeked(usize),
    Kind(moving_picture::Kind),
}

struct App {
    gif: Option<moving_picture::Frames>,
    apng: Option<moving_picture::Frames>,
    paused: bool,
    looping: bool,
    frame_index: usize,
    kind: moving_picture::Kind,
    speed: Speed,
}

impl Default for App {
    fn default() -> Self {
        Self {
            gif: None,
            apng: None,
            paused: false,
            looping: true,
            frame_index: 0,
            kind: moving_picture::Kind::Gif,
            speed: Speed(1.0),
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
            Message::ToggledLoop => {
                self.looping = !self.looping;
            }
            Message::SetSpeed(speed) => {
                self.speed = speed;
            }
            Message::NewFrame(index) => {
                self.frame_index = index;
            }
            Message::LastFrame => {
                println!("played last frame");
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

                let frames = match kind {
                    moving_picture::Kind::Apng => self.apng.as_mut(),
                    moving_picture::Kind::Gif => self.gif.as_mut(),
                };

                if let Some(frames) = frames {
                    frames.seek(moving_picture::Position::Frame(0));
                }
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
            let speed = f64::from(self.speed);

            let image = moving_picture(frames)
                .paused(self.paused)
                .looping(self.looping)
                .speed(speed)
                .on_new_frame(Message::NewFrame)
                .on_last_frame(|| Message::LastFrame);

            column![
                row![
                    pick_list(
                        vec![moving_picture::Kind::Gif, moving_picture::Kind::Apng],
                        Some(self.kind),
                        Message::Kind
                    ),
                    pick_list(
                        vec![Speed(0.5), Speed(1.0), Speed(2.0)],
                        Some(self.speed),
                        Message::SetSpeed
                    ),
                    button(text(if self.paused { "Resume" } else { "Pause" }))
                        .on_press(Message::ToggledPause),
                    button(text(if self.looping {
                        "Stop loop"
                    } else {
                        "Start loop"
                    }))
                    .on_press(Message::ToggledLoop),
                ]
                .spacing(8),
                row![
                    text(format!("frame: {}/{}", self.frame_index + 1, total)),
                    text(format!(
                        "time: {:.2}/{:.2}",
                        frames.duration_at(self.frame_index, speed).as_secs_f64(),
                        frames.duration(speed).as_secs_f64()
                    )),
                ]
                .spacing(8),
                slider(0.0..=((total - 1) as f64), self.frame_index as f64, |v| {
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
