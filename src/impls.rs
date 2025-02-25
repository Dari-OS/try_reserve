#![cfg(not(feature = "no_std"))]

//! # Impls
//!
//! All implementation for all std types that use `try_reserve()`

use crate::{error::TryReserveError, TryReserve};

impl<T> TryReserve for Vec<T> {
    fn try_reserve(&mut self, additional: usize) -> Result<(), crate::error::TryReserveError> {
        Vec::try_reserve(self, additional).map_err(TryReserveError::from)
    }
}

impl<T> TryReserve for std::collections::HashSet<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    fn try_reserve(&mut self, additional: usize) -> Result<(), crate::error::TryReserveError> {
        std::collections::HashSet::try_reserve(self, additional).map_err(TryReserveError::from)
    }
}

impl TryReserve for std::ffi::OsString {
    fn try_reserve(&mut self, additional: usize) -> Result<(), crate::error::TryReserveError> {
        std::ffi::OsString::try_reserve(self, additional).map_err(TryReserveError::from)
    }
}

impl TryReserve for std::path::PathBuf {
    fn try_reserve(&mut self, additional: usize) -> Result<(), crate::error::TryReserveError> {
        std::path::PathBuf::try_reserve(self, additional).map_err(TryReserveError::from)
    }
}

impl TryReserve for std::string::String {
    fn try_reserve(&mut self, additional: usize) -> Result<(), crate::error::TryReserveError> {
        std::string::String::try_reserve(self, additional).map_err(TryReserveError::from)
    }
}

impl<T> TryReserve for std::collections::VecDeque<T> {
    fn try_reserve(&mut self, additional: usize) -> Result<(), crate::error::TryReserveError> {
        std::collections::VecDeque::try_reserve(self, additional).map_err(TryReserveError::from)
    }
}

impl<T> TryReserve for std::collections::BinaryHeap<T> {
    fn try_reserve(&mut self, additional: usize) -> Result<(), crate::error::TryReserveError> {
        std::collections::BinaryHeap::try_reserve(self, additional).map_err(TryReserveError::from)
    }
}

impl<T, F> TryReserve for std::collections::HashMap<T, F>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    fn try_reserve(&mut self, additional: usize) -> Result<(), crate::error::TryReserveError> {
        std::collections::HashMap::try_reserve(self, additional).map_err(TryReserveError::from)
    }
}
