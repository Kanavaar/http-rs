#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    MethodConversion(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            Self::MethodConversion(method) => format!("Conversion of method {method} failed"),
        };

        write!(f, "{}", error)
    }
}
