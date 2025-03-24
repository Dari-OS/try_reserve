#![cfg_attr(not(feature = "std"), no_std)]
//! Stable TryReserveError that exposes TryReserveErrorKind
//!

pub mod error;
#[cfg(feature = "std")]
mod impls;
mod try_reserve;

pub use try_reserve::TryReserve;
