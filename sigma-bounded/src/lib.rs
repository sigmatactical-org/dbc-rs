//! Bounded [`Vec`], [`String`], and [`BTreeMap`] for `alloc` or [`heapless`].
//!
//! Part of the Sigma **`dbc-rs`** **embedded-first** story: pick **`alloc`** (default) or
//! **`heapless`** (`default-features = false`) so firmware policy matches **`dbc-rs`** without
//! duplicating collection wrappers elsewhere.

// Embedded-first: always `no_std`. `alloc`-backed collections pull in the
// `alloc` crate below; the `heapless` backend needs neither `std` nor `alloc`.
// (Previously this was `no_std` only in the non-`alloc` build, which made the
// `alloc` feature silently require `std` and broke bare-metal targets.)
#![no_std]
#![forbid(unsafe_code)]

#[cfg(all(not(feature = "alloc"), not(feature = "heapless")))]
compile_error!("Either the `alloc` or `heapless` feature must be enabled");

#[cfg(feature = "alloc")]
extern crate alloc;

mod btree_map;
pub mod error;
mod string;
mod vec;

pub use btree_map::BTreeMap;
pub use error::{Error, Result};
pub use string::String;
pub use vec::Vec;
