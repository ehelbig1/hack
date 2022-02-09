use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Request failed: {0}")]
    RequestFailed(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::RequestFailed(err.to_string())
    }
}
