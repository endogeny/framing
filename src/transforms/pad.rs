use super::super::Image;

/// Uses the given pixel to pad a frame.
///
/// The result will have a width of `left + frame.width + right` and a height of
/// `top + frame.height + bottom`.
pub fn pad<T: Image>(
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    padding: T::Pixel,
    frame: T
) -> Pad<T> {
    Pad { top, bottom, left, right, padding, frame }
}

#[doc(hidden)]
pub struct Pad<T: Image> {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    padding: T::Pixel,
    frame: T
}

impl<T: Image> Image for Pad<T>
where T::Pixel: Clone {
    type Pixel = T::Pixel;

    fn width(&self) -> usize { self.left + self.frame.width() + self.right }
    fn height(&self) -> usize { self.top + self.frame.height() + self.bottom }

    unsafe fn pixel(&self, mut x: usize, mut y: usize) -> Self::Pixel {
        if x < self.left || y < self.top {
            self.padding.clone()
        } else {
            x -= self.left;
            y -= self.top;

            if x < self.frame.width() && y < self.frame.height() {
                self.frame.pixel(x, y)
            } else {
                self.padding.clone()
            }
        }
    }
}
