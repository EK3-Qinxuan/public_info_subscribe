use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("transport error: {0}")]
    Transport(String),
    #[error("codec error: {0}")]
    Codec(String),
    #[error("normalization error: {0}")]
    Normalization(String),
    #[error("subscription error: {0}")]
    Subscription(String),
    #[error("other error: {0}")]
    Other(String),
}

impl Error {
    pub fn ctx(kind: &'static str, err: impl std::fmt::Display) -> Self {
        Self::Other(format!("{kind}: {err}"))
    }
}
