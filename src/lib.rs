#![feature(conservative_impl_trait)]
#![warn(missing_docs)]

//! Audio and video frames and some helpers to deal with composing them.

extern crate bytes;
extern crate clamp;
extern crate rayon;
extern crate time;

/// Video frames, formats, and basic operations involving them.
pub mod video;

// TODO(quadrupleslap): Audio.
// TODO(quadrupleslap): Composition and muxing helpers.
// TODO(quadrupleslap): FFmpeg bindings.
