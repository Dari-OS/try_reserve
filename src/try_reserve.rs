use crate::error::{TryReserveError, TryReserveErrorKind};

#[allow(dead_code)]
pub trait TryReserve {
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;
}

pub trait TryReserveErrorExtension {
    #[allow(dead_code)]
    fn to_crate_error(self) -> TryReserveError;
    #[allow(dead_code)]
    fn kind(self) -> TryReserveErrorKind;
}

#[cfg(not(feature = "no_std"))]
impl TryReserveErrorExtension for std::collections::TryReserveError {
    fn to_crate_error(self) -> TryReserveError {
        TryReserveError::from(self)
    }

    fn kind(self) -> TryReserveErrorKind {
        TryReserveError::from(self).kind()
    }
}
