pub mod apng;
pub mod gif;

pub use apng::Apng;
pub use gif::Gif;

/// Creates a new [`Gif`] with the given [`gif::Frames`]
pub fn gif(frames: &gif::Frames) -> Gif {
    Gif::new(frames)
}

/// Creates a new [`Apng`] with the given [`apng::Frames`]
pub fn apng(frames: &apng::Frames) -> Apng {
    Apng::new(frames)
}
