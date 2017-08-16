#![feature(conservative_impl_trait)]
#![warn(missing_docs)]

//! Fast image frames.

extern crate clamp;
extern crate rayon;

mod formats;
mod function;
mod chunky;
mod transforms;

pub use self::formats::*;
pub use self::transforms::*;
pub use self::chunky::*;
pub use self::function::*;
use std::ops::Deref;

/// A `Image` is just a still image.
///
/// Frames are made up of a number of pixels arranged in a dense and finite 2D
/// grid. The pixels can be any type, but are usually RGBA or YUV values. A
/// frame is usually backed by either a (potentially foreign) buffer or by
/// another frame, which it transforms in some way, such as by rotating or
/// inverting the pixels.
pub trait Image {
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

impl<T, U> Image for U
where
    U: Deref<Target = T> + ?Sized,
    T: Image + ?Sized
{
    type Pixel = T::Pixel;

    fn width(&self) -> usize { self.deref().width() }
    fn height(&self) -> usize { self.deref().height() }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        self.deref().pixel(x, y)
    }
}
