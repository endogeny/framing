use bytes::{Bytes, BytesMut};
use rayon::prelude::*;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicPtr, Ordering};
use super::{VideoFrame, ByteChannels};

/// A [planar frame](https://en.wikipedia.org/wiki/Planar_(computer_graphics)).
///
/// Each plane is made up of one-byte subpixels, and the planes are stored
/// contiguously in memory, in the order specified by the pixel itself. So, for
/// example, a BGRA planar frame would have all the B bytes in order, then all
/// the G bytes, then the R bytes, and finally the A bytes, with each plane
/// covering the entire image in row-major order.
#[derive(Clone, Debug)]
pub struct PlanarFrame<T> {
    bytes: Bytes,
    width: usize,
    height: usize,
    pixel: PhantomData<T>
}

impl<T: ByteChannels> VideoFrame for PlanarFrame<T> {
    type Pixel = T;

    fn width(&self)  -> usize { self.width }
    fn height(&self) -> usize { self.height }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        let len = self.width * self.height;
        let mut off = y * self.width + x;
        let mut channels = T::Channels::default();

        for channel in channels.as_mut() {
            *channel = *self.bytes.get_unchecked(off);
            off += len;
        }

        channels.into()
    }
}

impl<T: ByteChannels> PlanarFrame<T> {
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

    /// Creates a new frame using the given function to fill the buffer.
    /// It is guaranteed that the mapping will be called **exactly once** for
    /// each of the integers in the range `[0, width) * [0, height)`.
    pub fn new<U>(frame: U) -> Self
    where U: VideoFrame<Pixel = T> + Sync {
        let (width, height) = (frame.width(), frame.height());
        let length = width * height;
        let size = T::width() * length;

        let mut bytes = BytesMut::with_capacity(size);
        let ptr = AtomicPtr::new(bytes.as_mut_ptr());
        unsafe { bytes.set_len(size); }

        (0..length).into_par_iter().for_each(|i| unsafe {
            let channels = T::Channels::from({
                let (x, y) = (i % width, i / width);
                frame.pixel(x, y).into()
            });

            let mut ptr = ptr.load(Ordering::Relaxed).offset(i as _);
            for &channel in channels.as_ref() {
                *ptr = channel;
                ptr = ptr.offset(length as _);
            }
        });

        Self::from_bytes(width, height, bytes.freeze())
    }
}

#[test]
fn black() {
    use super::{Function, Rgba, pixels};

    let (w, h) = (1280, 720);
    let frame = PlanarFrame::new(
        Function::new(w, h, |_, _| Rgba(0, 0, 0, 0))
    );

    assert_eq!(frame.width(), w);
    assert_eq!(frame.height(), h);
    assert_eq!(frame.bytes, vec![0; w * h * 4]);
    assert_eq!(pixels(&frame).count(), w * h);
    assert!(pixels(&frame).all(|x| x == Rgba(0, 0, 0, 0)));
}
