use super::super::Image;

/// Crops the given frame to the given rectangle.
pub fn crop<T: Image>(start: (usize, usize), size: (usize, usize), frame: T) -> Crop<T> {
    Crop { start, size, frame }
}

#[doc(hidden)]
pub struct Crop<T> {
    start: (usize, usize),
    size: (usize, usize),
    frame: T,
}

impl<T: Image> Image for Crop<T> {
    type Pixel = T::Pixel;

    fn width(&self) -> usize {
        self.size.0
    }
    fn height(&self) -> usize {
        self.size.1
    }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        let (x0, y0) = self.start;
        self.frame.pixel(x + x0, y + y0)
    }
}
