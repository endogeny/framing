use super::VideoFrame;

/// A frame backed by a function.
///
/// This might be useful if you just wanted to hack together a fractal renderer,
/// or wanted to wrap an image coming from a C library. Using `move` closures
/// with this struct essentially lets you build anonymous frames.
#[derive(Clone, Debug)]
pub struct Function<T, F>
where F: Fn(usize, usize) -> T {
    width: usize,
    height: usize,
    func: F
}

impl<T, F> VideoFrame for Function<T, F>
where F: Fn(usize, usize) -> T{
    type Pixel = T;

    fn width(&self)  -> usize { self.width }
    fn height(&self) -> usize { self.height }

    unsafe fn pixel(&self, x: usize, y: usize) -> Self::Pixel {
        (self.func)(x, y)
    }
}

impl<T, F> Function<T, F>
where F: Fn(usize, usize) -> T {
    /// Creates a new frame.
    pub fn new(width: usize, height: usize, func: F) -> Self {
        Self { width, height, func }
    }
}

#[test]
fn sollux() {
    let sollux = Function::new(2, 1, |x, _| if x == 0 { 0 } else { 1 });
    unsafe {
        assert_eq!(sollux.pixel(0, 0), 0);
        assert_eq!(sollux.pixel(1, 0), 1);
    }
}
