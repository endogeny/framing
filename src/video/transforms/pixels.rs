use rayon::prelude::*;
use super::super::VideoFrame;

/// Iterates over the pixels of a frame in parallel.
pub fn pixels<'a, T>(frame: &'a T) -> impl ParallelIterator<Item = T::Pixel>
where T: VideoFrame + Sync, T::Pixel: Send {

    let (w, h) = (frame.width(), frame.height());

    (0..w * h).into_par_iter().map(move |i| {
        unsafe {
            frame.pixel(i % w, i / w)
        }
    })
}
