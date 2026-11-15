pub mod moving_picture;

pub use moving_picture::MovingPicture;

/// Creates a new [`MovingPicture`] with the given [`moving_picture::Frames`]
pub fn moving_picture<'a, Message>(
    frames: &'a moving_picture::Frames,
) -> MovingPicture<'a, Message> {
    MovingPicture::new(frames)
}
