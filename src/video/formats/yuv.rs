use clamp::clamp;
use super::{Rgba, ByteChannels};

/// A three-byte long pixel in the Y'CbCr format.
///
/// Conversion to and from RGBA follows the BT.709 standard for 8-bit digital
/// representation of Y'CbCr encoded pixels.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Yuv(pub u8, pub u8, pub u8);

unsafe impl ByteChannels for Yuv {
    type Channels = [u8; 3];
    fn width() -> usize { 3 }
}

impl From<[u8; 3]> for Yuv {
    fn from(c: [u8; 3]) -> Self {
        Yuv(c[0], c[1], c[2])
    }
}

impl From<Yuv> for [u8; 3] {
    fn from(p: Yuv) -> Self {
        [p.0, p.1, p.2]
    }
}

impl From<Rgba> for Yuv {
    fn from(Rgba(r, g, b, _): Rgba) -> Yuv {
        let (r, g, b) = (r as f32, g as f32, b as f32);

        let y =   0.183 * r + 0.614 * g + 0.062 * b;
        let u = - 0.101 * r - 0.339 * g + 0.439 * b;
        let v =   0.439 * r - 0.399 * g - 0.040 * b;

        Yuv(
            (y +  16.0) as u8,
            (u + 128.0) as u8,
            (v + 128.0) as u8
        )
    }
}

impl From<Yuv> for Rgba {
    fn from(Yuv(y, u, v): Yuv) -> Rgba {
        let (y, u, v) = (y as f32 - 16.0, u as f32 - 128.0, v as f32 - 128.0);

        let r = clamp(1.164 * y             + 1.793 * v, 0.0, 255.0);
        let g = clamp(1.164 * y - 0.213 * u - 0.533 * v, 0.0, 255.0);
        let b = clamp(1.164 * y + 2.112 * u            , 0.0, 255.0);

        Rgba(
            r as u8,
            g as u8,
            b as u8,
            255
        )
    }
}

#[test]
fn conversion() {
    const MAX_ERROR: u8 = 4;

    for r in 0..255 {
    for g in 0..255 {
    for b in 0..255 {
        let a = Rgba(r, g, b, 255);
        let b = Rgba::from(Yuv::from(a));

        let distchk = |x, y| {
            let dist = if y <= x { x - y } else { y - x };

            if dist > MAX_ERROR {
                panic!("{:?} is too far from {:?}!", b, a);
            }
        };

        distchk(a.0, b.0);
        distchk(a.1, b.1);
        distchk(a.2, b.2);
        assert_eq!(b.3, 255);
    }}}
}
