use clamp::clamp;
use super::{Rgba, AsBytes};

/// A pixel that is one byte long and has just an intensity channel.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Grayscale<T = u8>(pub T);

unsafe impl AsBytes for Grayscale {
    type Bytes = [u8; 1];
    fn width() -> usize { 1 }
}

impl From<[u8; 1]> for Grayscale {
    fn from(c: [u8; 1]) -> Self {
        Grayscale(c[0])
    }
}

impl From<Grayscale> for [u8; 1] {
    fn from(p: Grayscale) -> Self {
        [p.0]
    }
}

impl From<Rgba> for Grayscale {
    fn from(Rgba(r, g, b, _): Rgba) -> Grayscale {
        const K_B: f32 = 0.0722;
        const K_R: f32 = 0.2126;
        const K_G: f32 = 1.0 - K_B - K_R;

        let (r, g, b) = (r as f32, g as f32, b as f32);

        Grayscale(clamp(K_R * r + K_G * g + K_B * b, 0.0, 255.0) as u8)
    }
}

impl From<Grayscale> for Rgba {
    fn from(Grayscale(l): Grayscale) -> Rgba {
        Rgba(l, l, l, 255)
    }
}
