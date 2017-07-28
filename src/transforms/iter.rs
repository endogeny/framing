use rayon::prelude::*;
use super::super::Image;

/// Iterates over the pixels of a frame sequentially.
pub fn iter<'a, T>(frame: &'a T)
    -> impl Iterator<Item = (usize, usize, T::Pixel)>
where
    T: Image + Sync,
    T::Pixel: Send,
{
    let (w, h) = (frame.width(), frame.height());

    (0..w * h).map(move |i| unsafe {
        let (x, y) = (i % w, i / w);
        (x, y, frame.pixel(i % w, i / w))
    })
}

/// Iterates over the pixels of a frame in parallel.
pub fn par_iter<'a, T>(frame: &'a T)
    -> impl ParallelIterator<Item = (usize, usize, T::Pixel)>
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
