extern crate libc;
#[cfg(feature = "wayland")]
extern crate memmap;
#[cfg(feature = "x11")]
extern crate xcb;

pub mod xkb;
