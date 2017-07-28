mod rgba;
mod grayscale;
mod yuv;

pub use self::rgba::*;
pub use self::grayscale::*;
pub use self::yuv::*;

// TODO(quadrupleslap): `AsBytes` for non-`u8`-backed pixel types.
// TODO(quadrupleslap): Can't wait for constants in traits.

/// Describes a kind of pixel that is made up of a fixed number of bytes.
///
/// # Safety
///
/// Perfectly safe to implement as long as the length of the byte array is
/// always equal to `Self::width()`. When the bugs in associated constants are
/// ironed out this will hopefully use them, and be safe to implement.
pub unsafe trait AsBytes: Sized {
    /// The type that represents the collection of bytes.
    ///
    /// Unless you know better, this type should probably be `[u8; N]`, where
    /// `N` is the number of bytes in each pixel.
    type Bytes:
        AsRef<[u8]> + AsMut<[u8]>
        + Default
        + From<Self> + Into<Self>;

    /// The number of channels in each pixel.
    ///
    /// For example, `RGB` might have three bytes per pixel, and `RGBA` might
    /// have four channels per pixel. This should match the length of the
    /// `Bytes` type, unless you enjoy breaking perfectly good code.
    fn width() -> usize;
}
