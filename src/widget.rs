pub mod apng;
pub mod gif;

pub use apng::Apng;
pub use gif::Gif;

/// Creates a new [`Gif`] with the given [`gif::Frames`]
pub fn gif<'a, Message>(frames: &'a gif::Frames) -> Gif<'a, Message> {
    Gif::new(frames)
}

/// Creates a new [`Apng`] with the given [`apng::Frames`]
pub fn apng<'a, Message>(frames: &'a apng::Frames) -> Apng<'a, Message> {
    Apng::new(frames)
}
