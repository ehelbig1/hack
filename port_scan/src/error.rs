use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("DNS lookup failed: {0}")]
    DNSLookupFailed(String),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::DNSLookupFailed(err.to_string())
    }
}
