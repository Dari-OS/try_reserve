# try_reserve

[![Crates.io](https://img.shields.io/crates/v/try_reserve.svg)](https://crates.io/crates/try_reserve)
[![Documentation](https://docs.rs/try_reserve/badge.svg)](https://docs.rs/try_reserve)
[![Rust](https://img.shields.io/badge/rust-1.60%2B-blue.svg?logo=rust)](https://www.rust-lang.org)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/try_reserve.svg)](./LICENSE)

A stable implementation of `TryReserveError` that exposes `TryReserveErrorKind`.

## Overview

This crate provides a stable implementation of the `TryReserveError` and `TryReserveErrorKind` types, which are currently not fully exposed in the Rust standard library.  
This is a workaround for [rust-lang/rust#48043](https://github.com/rust-lang/rust/issues/48043), an RFC that has been pending for stabilization for 7 years.

## Purpose

The sole purpose of this crate is to expose the normally private `TryReserveErrorKind` enum.  
This allows for easier and unified creation of custom collections that need to return appropriate allocation error types.

## Features

- `no_std` support via feature flag
- Full compatibility with std's `TryReserveError` when available using `tansmutation`
- Complete integration with already existing types inside the std that use `try_reserve()`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
try_reserve = "0.1"
```

### Basic usage

```rust
use try_reserve::{TryReserve, error::{TryReserveError, TryReserveErrorKind}};

// Implement the TryReserve trait for your custom collection
struct MyCollection<T> {
    data: Vec<T>,
}

impl<T> TryReserve for MyCollection<T> {
fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
    // Attempt to reserve space
    self.data.try_reserve(additional)
    .map_err(TryReserveError::from)
    }
}

// Or create your own error with specific kinds
fn example() -> Result<(), TryReserveError> {
    // Create a capacity overflow error
    let overflow_error = TryReserveErrorKind::CapacityOverflow;
    Err(overflow_error.into())
}
```

### With `no_std`

To use this crate in a `no_std` environment, enable the `no_std` feature:

```toml
[dependencies]
try_reserve = { version = "0.1", features = ["no_std"] }
```

## Error Types

### `TryReserveError`

This is the main error type returned by `try_reserve` methods. It wraps a `TryReserveErrorKind` and provides a clean API.

### `TryReserveErrorKind`

An enum with the following variants:

- `CapacityOverflow`: Error due to the computed capacity exceeding the collection's maximum (usually `isize::MAX` bytes).
- `AllocError { layout }`: Error returned when the memory allocator fails. Contains the layout of the allocation request that failed.

## Conversions

When not in a `no_std` environment, the crate provides conversions between its error types and the standard library's error types:

```rust
use try_reserve::error::TryReserveError;

// Convert from std to this crate's error type
let std_error = Vec::<i32>::with_capacity(100)
    .try_reserve(usize::MAX)
    .unwrap_err();
let our_error = TryReserveError::from(std_error);
// or
// let our_error: TryReserveError = std_error.into();

// And back again
let std_error_again = std::collections::TryReserveError::from(our_error);

```

## Why this crate exists

The Rust standard library has had a pending RFC for the stabilization of the `TryReserveErrorKind` enum for 7 years (see [rust-lang/rust#48043](https://github.com/rust-lang/rust/issues/48043)).  
Without access to this enum, it's difficult to create custom collections that properly handle and report allocation errors.

This crate provides a stable workaround until the standard library stabilizes this API.

## License

Licensed under

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
