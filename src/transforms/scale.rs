use super::super::Image;

/// Scales a frame to the given size, using nearest-neighbor interpolation.
pub fn scale<T>(width: usize, height: usize, frame: T) -> Scale<T> {
    Scale {
        width,
        height,
        frame,
    }
}

#[doc(hidden)]
pub struct Scale<T> {
    width: usize,
    height: usize,
    frame: T,
}

impl<T: Image> Image for Scale<T> {
    type Pixel = T::Pixel;

    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        self.frame.pixel(
            x * self.frame.width() / self.width,
            y * self.frame.height() / self.height,
        )
    }
}
