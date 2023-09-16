use crate::pixel::Pixel;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Pixel for Rgb {
    type Component = u8;
    const COMPONENT_COUNT: usize = 3;

    fn from_slice(slice: &[Self::Component]) -> Self {
        Self(slice[0], slice[1], slice[2])
    }
}
