use crate::pixel::Pixel;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb<T>(pub T, pub T, pub T);

impl<T: Copy + Default> Pixel for Rgb<T> {
    type Component = T;
    const COMPONENT_COUNT: usize = 3;
    fn from_slice(slice: &[Self::Component]) -> Self {
        Self(slice[0], slice[1], slice[2])
    }
}
