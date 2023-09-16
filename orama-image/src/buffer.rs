use crate::pixel::Pixel;

/// A buffer of pixels.
pub struct Buffer<T: Pixel> {
    width: u32,
    height: u32,
    data: Vec<T::Component>,
}

impl<T> Buffer<T>
where
    T: Pixel,
{
    /// Create a new buffer with the given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize * T::COMPONENT_COUNT;
        Self {
            width,
            height,
            data: vec![T::Component::default(); size],
        }
    }

    /// Get the width of the buffer.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the buffer.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get the pixel at the given coordinates.
    pub fn at(&self, x: u32, y: u32) -> T {
        let index = (y * self.width + x) as usize * T::COMPONENT_COUNT;
        T::from_slice(&self.data[index..index + T::COMPONENT_COUNT])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rgb::Rgb;

    #[test]
    fn test_buffer() {
        let buffer = Buffer::<Rgb>::new(10, 10);
        assert_eq!(buffer.width(), 10);
        assert_eq!(buffer.height(), 10);
        assert_eq!(buffer.at(0, 0), Rgb(0, 0, 0));
    }
}
