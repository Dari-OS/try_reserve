use core::{
    alloc::{Layout, LayoutError},
    fmt::Display,
};

#[cfg(feature = "std")]
use core::mem::transmute;

/// The error type for `try_reserve` methods.
///
/// This error is returned when memory allocation fails or when the capacity
/// exceeds collection-specific limits.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryReserveError {
    kind: TryReserveErrorKind,
}

impl TryReserveError {
    /// Returns the kind of allocation error that occurred.
    #[inline]
    #[allow(dead_code)]
    pub fn kind(&self) -> TryReserveErrorKind {
        self.kind.clone()
    }

    /// Converts a standard library `TryReserveError` into this crate's version.
    ///
    /// This is only available when the `no_std` feature is not enabled.
    #[cfg(feature = "std")]
    pub fn from_std(error: std::collections::TryReserveError) -> Self {
        Self::from(error)
    }

    /// Converts a `Result` containing a standard library `TryReserveError` into a `Result`
    /// with this crate's version of `TryReserveError`.
    ///
    /// This is only available when the `no_std` feature is not enabled.
    #[cfg(feature = "std")]
    pub fn from_std_result<T>(
        result: Result<T, std::collections::TryReserveError>,
    ) -> Result<T, Self> {
        result.map_err(Self::from_std)
    }
}

/// Details of the allocation that caused a `TryReserveError`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TryReserveErrorKind {
    /// Error due to the computed capacity exceeding the collection's maximum
    /// (usually `isize::MAX` bytes).
    CapacityOverflow,

    /// The memory allocator returned an error
    #[allow(dead_code)]
    AllocError {
        /// The layout of allocation request that failed
        layout: Layout,

        /// Reserved field for future compatibility with the standard library.
        /// This aligns with an RFC for future allocator error handling:
        /// https://github.com/rust-lang/wg-allocators/issues/23
        /// Ensures compatibility when in std environment
        #[cfg(feature = "std")]
        non_exhaustive: (),
    },
}

impl From<TryReserveErrorKind> for TryReserveError {
    /// Creates a `TryReserveError` from a `TryReserveErrorKind`.
    #[inline]
    fn from(kind: TryReserveErrorKind) -> Self {
        Self { kind }
    }
}

impl From<LayoutError> for TryReserveErrorKind {
    /// Always evaluates to [`TryReserveErrorKind::CapacityOverflow`].
    ///
    /// This conversion is used when a layout error occurs during allocation.
    #[inline]
    fn from(_: LayoutError) -> Self {
        TryReserveErrorKind::CapacityOverflow
    }
}

impl From<LayoutError> for TryReserveError {
    /// Always evaluates to a `TryReserveError` with [`TryReserveErrorKind::CapacityOverflow`].
    ///
    /// This conversion is used when a layout error occurs during allocation.
    fn from(_: LayoutError) -> Self {
        TryReserveError {
            kind: TryReserveErrorKind::CapacityOverflow,
        }
    }
}

#[cfg(feature = "std")]
impl From<std::collections::TryReserveError> for TryReserveError {
    /// Converts a standard library `TryReserveError` into this crate's version.
    ///
    /// Uses direct memory transmutation since the internal structure is identical.
    fn from(value: std::collections::TryReserveError) -> Self {
        unsafe { transmute::<std::collections::TryReserveError, TryReserveError>(value) }
    }
}

#[cfg(feature = "std")]
impl From<TryReserveErrorKind> for std::collections::TryReserveError {
    /// Converts a `TryReserveErrorKind` into a standard library `TryReserveError`.
    ///
    /// Creates a `TryReserveError` first, then converts it to the standard library version.
    fn from(value: TryReserveErrorKind) -> Self {
        TryReserveError { kind: value }.into()
    }
}

#[cfg(feature = "std")]
impl From<TryReserveError> for std::collections::TryReserveError {
    /// Converts this crate's `TryReserveError` into a standard library version.
    ///
    /// Uses direct memory transmutation since the internal structure is identical.
    fn from(val: TryReserveError) -> Self {
        unsafe { transmute::<TryReserveError, std::collections::TryReserveError>(val) }
    }
}

impl Display for TryReserveError {
    /// Formats the error message for display.
    ///
    /// Provides information about why the allocation failed.
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        fmt.write_str("memory allocation failed")?;
        let reason = match self.kind {
            TryReserveErrorKind::CapacityOverflow => {
                " because the computed capacity exceeded the collection's maximum"
            }
            TryReserveErrorKind::AllocError { .. } => {
                " because the memory allocator returned an error"
            }
        };
        fmt.write_str(reason)
    }
}

// To be honest I have absolutly no idea why this trait exists or what it does
// Searching for it on the internet only shows source-code and the doc underneath
//
//
///// An intermediate trait for specialization of `Extend`.
//#[cfg(not(feature = "no_global_oom_handling"))]
//#[allow(dead_code)]
//trait SpecExtend<I: IntoIterator> {
//    /// Extends `self` with the contents of the given iterator.
//    fn spec_extend(&mut self, iter: I);
//}

/// Implements the standard error trait for `TryReserveError`.
///
/// This enables using this error type with standard error handling mechanisms.
impl core::error::Error for TryReserveError {}
