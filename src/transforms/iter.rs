use super::super::Image;
use rayon::prelude::*;

/// Iterates over the pixels of a frame sequentially.
pub fn iter<T: Image>(
    frame: &T,
) -> impl Iterator<Item = (usize, usize, T::Pixel)> + ExactSizeIterator + '_ {
    let (w, h) = (frame.width(), frame.height());

    (0..w * h).map(move |i| unsafe {
        let (x, y) = (i % w, i / w);
        (x, y, frame.pixel(i % w, i / w))
    })
}

/// Iterates over the pixels of a frame in parallel.
pub fn par_iter<T>(frame: &T) -> impl ParallelIterator<Item = (usize, usize, T::Pixel)> + '_
where
    T: Image + Sync,
    T::Pixel: Send,
{
    let (w, h) = (frame.width(), frame.height());

    (0..w * h).into_par_iter().map(move |i| unsafe {
        let (x, y) = (i % w, i / w);
        (x, y, frame.pixel(i % w, i / w))
    })
}
