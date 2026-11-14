pub mod widget;

use std::time::Duration;

pub use widget::{apng, apng::Apng, gif, gif::Gif};

#[derive(Debug)]
pub enum Position {
    Frame(usize),
    Time(Duration),
}
