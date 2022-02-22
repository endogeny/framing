use super::AsBytes;

/// A pixel that is four bytes long and is made of a red, green, blue and alpha
/// channel, in that order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rgba<T = u8>(pub T, pub T, pub T, pub T);

/// A pixel that is four bytes long and is made of a blue, green, red and alpha
/// channel, in that order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Bgra<T = u8>(pub T, pub T, pub T, pub T);

impl<T> From<Bgra<T>> for Rgba<T> {
    fn from(Bgra(b, g, r, a): Bgra<T>) -> Self {
        Rgba(r, g, b, a)
    }
}

impl<T> From<Rgba<T>> for Bgra<T> {
    fn from(Rgba(r, g, b, a): Rgba<T>) -> Self {
        Bgra(b, g, r, a)
    }
}

macro_rules! four {
    ($x:ident) => {
        unsafe impl AsBytes for $x {
            type Bytes = [u8; 4];
            fn width() -> usize {
                4
            }
        }

        impl From<[u8; 4]> for $x {
            fn from(c: [u8; 4]) -> Self {
                $x(c[0], c[1], c[2], c[3])
            }
        }

        impl From<$x> for [u8; 4] {
            fn from(p: $x) -> Self {
                [p.0, p.1, p.2, p.3]
            }
        }
    };
}

four!(Rgba);
four!(Bgra);
