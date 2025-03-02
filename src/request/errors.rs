use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Request failed to send: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Failed to decode JSON: {0}")]
    SerdeDecodeFailed(#[from] serde_json::Error),
    #[error("No course found that matches the search criteria")]
    NoCourseFound,
    #[error("Unknown error returned: {0}")]
    UnknownError(String),
}