#![cfg_attr(feature = "no_std", no_std)]
//! Stable TryReserveError that exposes TryReserveErrorKind
//!

pub mod error;
#[cfg(not(feature = "no_std"))]
mod impls;
mod try_reserve;

pub use try_reserve::TryReserve;
