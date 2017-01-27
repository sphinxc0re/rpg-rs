use super::Sprite;

/// An animated sprite
pub struct AnimatedSprite {
    frames: Vec<Sprite>,
    current_frame: usize,
    started: bool,
}
