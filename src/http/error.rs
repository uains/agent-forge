use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),

    #[error("Request build error: {0}")]
    RequestBuild(#[from] http::Error),

    #[error("HTTP transport error: {0}")]
    Transport(#[from] hyper_util::client::legacy::Error),

    #[error("Response body error: {0}")]
    Body(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP status error: {status}")]
    Status {
        status: http::StatusCode,
        body: String,
    },

    #[error("Request timeout")]
    Timeout,

    #[error("Read idle timeout")]
    ReadIdleTimeout,

    #[error("Response body exceeds configured limit")]
    BodyToolarge,
}
