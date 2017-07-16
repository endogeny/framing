mod rgba;
mod grayscale;
mod yuv;

pub use self::rgba::*;
pub use self::grayscale::*;
pub use self::yuv::*;

// TODO(quadrupleslap): Can't wait for constants in traits.

/// Describes a kind of pixel that is made up of a finite number of channels,
/// each one byte long.
pub unsafe trait ByteChannels: Sized {
    /// The type that represents the collection of channels.
    ///
    /// Unless you know better, this type should probably be `[u8; N]`, where
    /// `N` is the number of channels in each pixel.
    type Channels:
        AsRef<[u8]> + AsMut<[u8]>
        + Default
        + From<Self> + Into<Self>;

    /// The number of channels in each pixel.
    ///
    /// For example, `RGB` might have three channels per pixel, and `RGBA` might
    /// have four channels per pixel. This should match the length of the
    /// `Channels` type, unless you enjoy breaking perfectly good code.
    fn width() -> usize;
}
