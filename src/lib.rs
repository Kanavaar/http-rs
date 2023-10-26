pub mod error;
pub mod header;
pub mod method;
pub mod url;

pub use crate::error::Error;
pub use crate::method::Method;

pub type HttpResult<T> = core::result::Result<T, Error>;
