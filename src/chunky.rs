use super::{AsBytes, Image};
use rayon::prelude::*;
use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

/// A [chunky frame](https://en.wikipedia.org/wiki/Packed_pixel).
///
/// In this format, each pixel is stored contiguously, and the entire image is
/// stored in row-major order. For example, this means that an RGBA image would
/// store the RGBA values of the top-left pixel, then each of the RGBA values of
/// the pixel immediately to the right, and so on, moving down through each row.
#[derive(Clone, Debug)]
pub struct Chunky<T, V = Vec<u8>> {
    bytes: V,
    width: usize,
    height: usize,
    pixel: PhantomData<T>,
}

impl<T, V> Image for Chunky<T, V>
where
    T: AsBytes,
    V: AsRef<[u8]>,
{
    type Pixel = T;

    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        let off = T::width() * (y * self.width + x);
        let mut bytes = T::Bytes::default();

        ptr::copy_nonoverlapping(
            self.bytes.as_ref().as_ptr().offset(off as isize),
            bytes.as_mut().as_mut_ptr(),
            T::width(),
        );

        bytes.into()
    }
}

impl<T, V> Chunky<T, V> {
    /// Creates a new frame backed by the provided byte source.
    ///
    /// # Panics
    ///
    /// Panics if the length of the buffer is not
    /// `width * height * bytes_per_pixel`.
    pub fn from_bytes(width: usize, height: usize, bytes: V) -> Self
    where
        T: AsBytes,
        V: AsRef<[u8]>,
    {
        assert_eq!(bytes.as_ref().len(), width * height * T::width());
        Self {
            bytes,
            width,
            height,
            pixel: PhantomData,
        }
    }

    /// Returns a read-only view into the frame's byte source.
    pub fn bytes(&self) -> &V {
        &self.bytes
    }

    /// Recovers the byte source.
    pub fn into_bytes(self) -> V {
        self.bytes
    }

    /// Returns a mutable view into the frame's byte source.
    pub fn bytes_mut(&mut self) -> &mut [u8]
    where
        V: AsMut<[u8]>,
    {
        self.bytes.as_mut()
    }

    /// Set the frame's contents to that of the given frame.
    ///
    /// # Panics
    ///
    /// Panics if the width and height of the given frame are not exactly the
    /// same as the width and height of the chunky frame.
    pub fn copy_from<U>(&mut self, frame: U)
    where
        T: AsBytes,
        U: Image<Pixel = T> + Sync,
        V: AsMut<[u8]>,
    {
        assert_eq!(frame.width(), self.width);
        assert_eq!(frame.height(), self.height);

        let length = self.width * self.height;
        let ptr = AtomicPtr::new(self.bytes.as_mut().as_mut_ptr());
        let width = self.width;

        (0..length).into_par_iter().for_each(|i| unsafe {
            let ptr = ptr.load(Ordering::Relaxed);
            let (x, y) = (i % width, i / width);
            let bytes = T::Bytes::from(frame.pixel(x, y).into());

            ptr::copy_nonoverlapping(
                bytes.as_ref().as_ptr(),
                ptr.offset((T::width() * i) as _),
                T::width(),
            );
        });
    }
}

impl<T> Chunky<T>
where
    T: AsBytes,
{
    /// Creates a new frame using the given frame to fill the buffer.
    /// It is guaranteed that the mapping will be called **exactly once** for
    /// each of the integers in the range `[0, width) * [0, height)`.
    pub fn new<U>(frame: U) -> Self
    where
        U: Image<Pixel = T> + Sync,
    {
        let (width, height) = (frame.width(), frame.height());
        let length = width * height;
        let size = T::width() * length;

        let mut bytes = Vec::with_capacity(size);
        unsafe {
            bytes.set_len(size);
        }

        let mut chunky = Self::from_bytes(width, height, bytes);
        chunky.copy_from(frame);
        chunky
    }
}

#[test]
fn black() {
    use super::{iter, Function, Rgba};

    let (w, h) = (1920, 1080);
    let frame = Chunky::new(Function::new(w, h, |_, _| Rgba(0, 0, 0, 0)));

    assert_eq!(frame.width(), w);
    assert_eq!(frame.height(), h);
    assert_eq!(frame.bytes, vec![0; w * h * 4]);
    assert_eq!(iter(&frame).count(), w * h);
    assert!(iter(&frame).all(|(_, _, x)| x == Rgba(0, 0, 0, 0)));
}
