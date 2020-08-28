use thiserror::Error;

#[derive(Debug, Error)]
pub enum APIError {
    #[error("Error: {0:?}")]
    Error(#[from] anyhow::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] ::std::io::Error),

    #[error("Job {0} not found")]
    JobNotFound(String),
}
