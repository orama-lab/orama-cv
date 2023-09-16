pub trait Pixel {
    // The type of data that is used to store each channel of the pixel.
    type Component;
    // The number of channels in the pixel.
    const CHANNEL_COUNT: usize;
}
