mod formats;
mod function;
mod chunky;
mod planar;
mod transforms;

pub use self::formats::*;
pub use self::transforms::*;
pub use self::chunky::*;
pub use self::planar::*;
pub use self::function::*;

/// A `VideoFrame` is just a still image.
///
/// Frames are made up of a number of pixels arranged in a dense and finite 2D
/// grid. The pixels can be any type, but are usually RGBA or YUV values. A
/// frame is usually backed by either a (potentially foreign) buffer or by
/// another frame, which it transforms in some way, such as by rotating or
/// inverting the pixels.
pub trait VideoFrame {
    /// The kind of pixel that the frame is made of.
    type Pixel;

    /// The width of the frame in pixels.
    fn width(&self)  -> usize;

    /// The height of the frame in pixels.
    fn height(&self) -> usize;

    /// Gets the pixel at the specified zero-indexed coordinates.
    ///
    /// # Undefined Behavior
    ///
    /// The caller must ensure that `0 <= x < width` and that `0 <= y < height`.
    /// If these invariants are not upheld, the callee is allowed to do whatever
    /// it wants to, including eat your laundry, panic, or return a result
    /// that makes no sense whatsoever.
    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel;
}

impl<'a, T> VideoFrame for &'a T where T: VideoFrame {
    type Pixel = T::Pixel;

    fn width(&self) -> usize { (*self).width() }
    fn height(&self) -> usize { (*self).height() }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        (*self).pixel(x, y)
    }
}
