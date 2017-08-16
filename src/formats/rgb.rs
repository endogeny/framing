use super::AsBytes;

/// A pixel that is three bytes long and is made of a red, green and blue
/// channel, in that order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rgb<T = u8>(pub T, pub T, pub T);

/// A pixel that is four bytes long and is made of a blue, green and red
/// channel, in that order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Bgr<T = u8>(pub T, pub T, pub T);

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
