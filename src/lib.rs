pub mod error;
pub mod method;

pub use crate::error::Error;
pub use crate::method::Method;

pub type HttpResult<T> = core::result::Result<T, Error>;
