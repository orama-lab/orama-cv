use crate::Buffer;
use crate::Format;
use crate::Rgb;

pub type RgbImage<T> = Buffer<Rgb<T>>;

#[non_exhaustive]
pub enum Image {
    Rgb8(RgbImage<u8>),
}

// Construction interface.
impl Image {
    pub fn new(width: u32, height: u32, format: Format) -> Self {
        match format {
            Format::Rgb => Self::Rgb8(RgbImage::<u8>::new(width, height)),
        }
    }

    pub fn from_slice(width: u32, height: u32, format: Format, data: &[u8]) -> Self {
        match format {
            Format::Rgb => Self::Rgb8(RgbImage::<u8>::from_slice(width, height, data)),
        }
    }

    pub fn from_vec(width: u32, height: u32, format: Format, data: Vec<u8>) -> Self {
        match format {
            Format::Rgb => Self::Rgb8(RgbImage::<u8>::from_vec(width, height, data)),
        }
    }
}

// Access interface.
impl Image {
    pub fn width(&self) -> u32 {
        match self {
            Self::Rgb8(image) => image.width(),
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Self::Rgb8(image) => image.height(),
        }
    }

    pub fn at(&self, x: u32, y: u32) -> Rgb<u8> {
        match self {
            Self::Rgb8(image) => image.at(x, y),
        }
    }
}
