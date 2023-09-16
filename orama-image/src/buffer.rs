use crate::pixel::Pixel;

pub struct Buffer<T: Pixel> {
    width: u32,
    height: u32,
}

impl<T> Buffer<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
