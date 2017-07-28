use super::AsBytes;

/// A pixel that is four bytes long and is made of a red, green, blue and alpha
/// channel, in that order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rgba<T = u8>(pub T, pub T, pub T, pub T);

unsafe impl AsBytes for Rgba {
    type Bytes = [u8; 4];
    fn width() -> usize { 4 }
}

impl From<[u8; 4]> for Rgba {
    fn from(c: [u8; 4]) -> Self {
        Rgba(c[0], c[1], c[2], c[3])
    }
}

impl From<Rgba> for [u8; 4] {
    fn from(p: Rgba) -> Self {
        [p.0, p.1, p.2, p.3]
    }
}
