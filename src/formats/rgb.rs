use super::{AsBytes, Rgba};

/// A pixel that is three bytes long and is made of a red, green and blue
/// channel, in that order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rgb<T = u8>(pub T, pub T, pub T);

/// A pixel that is three bytes long and is made of a blue, green and red
/// channel, in that order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Bgr<T = u8>(pub T, pub T, pub T);

impl<T> From<Rgba<T>> for Rgb<T> {
    fn from(Rgba(r, g, b, _): Rgba<T>) -> Self {
        Rgb(r, g, b)
    }
}

impl<T> From<Rgba<T>> for Bgr<T> {
    fn from(Rgba(r, g, b, _): Rgba<T>) -> Self {
        Bgr(b, g, r)
    }
}

impl From<Rgb> for Rgba {
    fn from(Rgb(r, g, b): Rgb) -> Self {
        Rgba(r, g, b, 255)
    }
}

impl From<Bgr> for Rgba {
    fn from(Bgr(b, g, r): Bgr) -> Self {
        Rgba(r, g, b, 255)
    }
}

macro_rules! three {
    ($x:ident) => {
        unsafe impl AsBytes for $x {
            type Bytes = [u8; 3];
            fn width() -> usize { 3 }
        }

        impl From<[u8; 3]> for $x {
            fn from(c: [u8; 3]) -> Self {
                $x(c[0], c[1], c[2])
            }
        }

        impl From<$x> for [u8; 3] {
            fn from(p: $x) -> Self {
                [p.0, p.1, p.2]
            }
        }
    }
}

three!(Rgb);
three!(Bgr);
