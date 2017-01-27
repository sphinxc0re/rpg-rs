/// A sprite that can be drawn onto a graphic context
pub struct Sprite {
    filename: String,
    width: u32,
    height: u32,
    content: Vec<u8>,
}
