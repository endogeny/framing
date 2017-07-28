use super::super::Image;

/// Constructs a pointwise mapping over a frame.
///
/// You can use this to, for example, convert between pixel formats, or invert
/// all the colors.
pub fn map<T, F, U>(map: F, frame: T) -> Map<T, F, U>
where T: Image, F: Fn(T::Pixel) -> U {
    Map { map, frame }
}

#[doc(hidden)]
pub struct Map<T, F, U>
where T: Image, F: Fn(T::Pixel) -> U {
    map: F,
    frame: T
}

impl<T, F, U> Image for Map<T, F, U>
where T: Image, F: Fn(T::Pixel) -> U {
    type Pixel = U;

    fn width(&self) -> usize { self.frame.width() }
    fn height(&self) -> usize { self.frame.height() }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        (self.map)(self.frame.pixel(x, y))
    }
}
