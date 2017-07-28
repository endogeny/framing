use bytes::{Bytes, BytesMut};
use rayon::prelude::*;
use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use super::{Image, AsBytes};

/// A [chunky frame](https://en.wikipedia.org/wiki/Packed_pixel).
///
/// In this format, each pixel is stored contiguously, and the entire image is
/// stored in row-major order. For example, this means that an RGBA image would
/// store the RGBA values of the top-left pixel, then each of the RGBA values of
/// the pixel immediately to the right, and so on, moving down through each row.
#[derive(Clone, Debug)]
pub struct ChunkyFrame<T> {
    bytes: Bytes,
    width: usize,
    height: usize,
    pixel: PhantomData<T>
}

impl<T> Image for ChunkyFrame<T> where T: AsBytes {
    type Pixel = T;

    fn width(&self)  -> usize { self.width }
    fn height(&self) -> usize { self.height }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        let off = T::width() * (y * self.width + x);
        let mut bytes = T::Bytes::default();

        ptr::copy_nonoverlapping(
            self.bytes.as_ptr().offset(off as isize),
            bytes.as_mut().as_mut_ptr(),
            T::width()
        );

        bytes.into()
    }
}

impl<T> ChunkyFrame<T> where T: AsBytes {
    /// Creates a new frame backed by the provided byte source.
    ///
    /// # Panics
    ///
    /// Panics if the length of the buffer is not
    /// `width * height * bytes_per_pixel`.
    pub fn from_bytes(
        width: usize,
        height: usize,
        bytes: Bytes
    ) -> Self {
        assert_eq!(bytes.len(), width * height * T::width());
        Self { bytes, width, height, pixel: PhantomData }
    }

    /// Returns a read-only view into the frame's byte source.
    ///
    /// This function is not as slow as you'd expect, because `Bytes` is
    /// actually reference-counted.
    pub fn bytes(&self) -> Bytes {
        self.bytes.clone()
    }

    /// Creates a new frame using the given frame to fill the buffer.
    /// It is guaranteed that the mapping will be called **exactly once** for
    /// each of the integers in the range `[0, width) * [0, height)`.
    pub fn new<U>(frame: U) -> Self
    where U: Image<Pixel = T> + Sync {
        let (width, height) = (frame.width(), frame.height());
        let length = width * height;
        let size = T::width() * length;

        let mut bytes = BytesMut::with_capacity(size);
        let ptr = AtomicPtr::new(bytes.as_mut_ptr());
        unsafe { bytes.set_len(size); }

        (0..length).into_par_iter().for_each(|i| unsafe {
            let ptr = ptr.load(Ordering::Relaxed);
            let (x, y) = (i % width, i / width);

            let bytes = T::Bytes::from(frame.pixel(x, y).into());

            ptr::copy_nonoverlapping(
                bytes.as_ref().as_ptr(),
                ptr.offset((T::width() * i) as _),
                T::width()
            );
        });

        Self::from_bytes(width, height, bytes.freeze())
    }
}

#[test]
fn black() {
    use super::{Function, Rgba, iter};

    let (w, h) = (1920, 1080);
    let frame = ChunkyFrame::new(
        Function::new(w, h, |_, _| Rgba(0, 0, 0, 0))
    );

    assert_eq!(frame.width(), w);
    assert_eq!(frame.height(), h);
    assert_eq!(frame.bytes, vec![0; w * h * 4]);
    assert_eq!(iter(&frame).count(), w * h);
    assert!(iter(&frame).all(|(_, _, x)| x == Rgba(0, 0, 0, 0)));
}
