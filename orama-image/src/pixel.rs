pub trait Pixel {
    // The type of data that is used to store each component of the pixel.
    type Component: Copy + Default;
    // The number of channels in the pixel.
    const COMPONENT_COUNT: usize;
    // Create a new pixel with the given components.
    fn from_slice(slice: &[Self::Component]) -> Self;
}
