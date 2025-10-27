use thiserror::Error;

#[derive(Error, Debug)]
pub enum KPSError {
    #[error("validation: {0}")]
    Validation(String),

    #[error("authentication: {0}")]
    Authentication(String),

    #[error("sts error: {0}")]
    STS(String),

    #[error("service error: {0}")]
    Service(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("timeout")]
    Timeout,

    #[error("other: {0}")]
    Other(String),
}
