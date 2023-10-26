pub mod error;
pub mod header;
pub mod method;
pub mod request;
pub mod url;

pub use crate::error::Error;
pub use crate::method::Method;
pub use crate::request::Request;

pub type HttpResult<T> = core::result::Result<T, Error>;
