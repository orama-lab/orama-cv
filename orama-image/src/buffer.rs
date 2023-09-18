use crate::pixel::Pixel;

/// A buffer of pixels.
pub struct Buffer<T: Pixel> {
    width: u32,
    height: u32,
    data: Vec<T::Component>,
}

// Construction interface.
impl<T: Pixel> Buffer<T> {
    /// Create a new buffer with the given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize * T::COMPONENT_COUNT;
        Self {
            width,
            height,
            data: vec![T::Component::default(); size],
        }
    }

    /// Create a new buffer from the given slice.
    pub fn from_slice(width: u32, height: u32, data: &[T::Component]) -> Self {
        assert_eq!(
            data.len(),
            (width * height) as usize * T::COMPONENT_COUNT,
            "Invalid buffer size"
        );
        Self {
            width,
            height,
            data: data.to_vec(),
        }
    }

    /// Create a new buffer from the given vector.
    pub fn from_vec(width: u32, height: u32, data: Vec<T::Component>) -> Self {
        assert_eq!(
            data.len(),
            (width * height) as usize * T::COMPONENT_COUNT,
            "Invalid buffer size"
        );
        Self {
            width,
            height,
            data,
        }
    }
}

// Access interface.
impl<T: Pixel> Buffer<T> {
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
    use crate::format::rgb::Rgb;

    #[test]
    fn test_buffer() {
        let buffer = Buffer::<Rgb<u8>>::new(10, 10);
        assert_eq!(buffer.width(), 10);
        assert_eq!(buffer.height(), 10);
        assert_eq!(buffer.at(0, 0), Rgb(0, 0, 0));
    }
}
