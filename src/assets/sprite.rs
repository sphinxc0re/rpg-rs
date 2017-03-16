use errors::*;
use super::Asset;
use std::path::Path;

/// A sprite that can be drawn onto a graphic context
pub struct Sprite {
    filename: String,
    width: u32,
    height: u32,
    content: Vec<u8>,
}

impl Asset for Sprite {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self>
        where Self: ::std::marker::Sized
    {
        Ok(Sprite {
            filename: stringify!(path).to_string(),
            width: 50,
            height: 50,
            content: Vec::new(),
        })
    }
}
