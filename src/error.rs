use core::{
    alloc::{Layout, LayoutError},
    fmt::Display,
};

#[cfg(not(feature = "no_std"))]
use core::mem::transmute;

/// The error type for `try_reserve` methods.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryReserveError {
    kind: TryReserveErrorKind,
}

impl TryReserveError {
    #[inline]
    #[allow(dead_code)]
    pub fn kind(&self) -> TryReserveErrorKind {
        self.kind.clone()
    }

    #[cfg(not(feature = "no_std"))]
    pub fn from_std(error: std::collections::TryReserveError) -> Self {
        Self::from(error)
    }

    #[cfg(not(feature = "no_std"))]
    pub fn from_std_result<T>(
        result: Result<T, std::collections::TryReserveError>,
    ) -> Result<T, Self> {
        result.map_err(Self::from_std)
    }
}

/// Details of the allocation that caused a `TryReserveError`
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

        // The std has this field for some future rfc: https://github.com/rust-lang/wg-allocators/issues/23"
        // I have to add it for the transmute methode
        non_exhaustive: (),
    },
}

impl From<TryReserveErrorKind> for TryReserveError {
    #[inline]
    fn from(kind: TryReserveErrorKind) -> Self {
        Self { kind }
    }
}

impl From<LayoutError> for TryReserveErrorKind {
    /// Always evaluates to [`TryReserveErrorKind::CapacityOverflow`].
    #[inline]
    fn from(_: LayoutError) -> Self {
        TryReserveErrorKind::CapacityOverflow
    }
}

impl From<LayoutError> for TryReserveError {
    /// Always evaluates to [`TryReserveErrorKind::CapacityOverflow`].
    fn from(_: LayoutError) -> Self {
        TryReserveError {
            kind: TryReserveErrorKind::CapacityOverflow,
        }
    }
}

#[cfg(not(feature = "no_std"))]
impl From<std::collections::TryReserveError> for TryReserveError {
    fn from(value: std::collections::TryReserveError) -> Self {
        unsafe { transmute::<std::collections::TryReserveError, TryReserveError>(value) }
    }
}

#[cfg(not(feature = "no_std"))]
impl From<TryReserveErrorKind> for std::collections::TryReserveError {
    fn from(value: TryReserveErrorKind) -> Self {
        TryReserveError { kind: value }.into()
    }
}

#[cfg(not(feature = "no_std"))]
impl From<TryReserveError> for std::collections::TryReserveError {
    fn from(val: TryReserveError) -> Self {
        unsafe { transmute::<TryReserveError, std::collections::TryReserveError>(val) }
    }
}

impl Display for TryReserveError {
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

impl core::error::Error for TryReserveError {}
