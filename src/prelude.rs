pub use crate::error::PorsmoError;
pub use crate::timers::*;

pub type Result<T> = core::result::Result<T, PorsmoError>;
