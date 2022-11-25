use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("File IO error {0}")]
    FileIOError(String),

    #[error("Queue error {0}")]
    QueueError(String),

    #[error("Http Error {0}")]
    HttpError(String),

    #[error("Metrics Type error {0}")]
    MetricsTypeError(String),

    #[error("Custom Error {0}")]
    CustomError(String),
}

impl From<std::io::Error> for ClientError {
    fn from(value: std::io::Error) -> Self {
        ClientError::FileIOError(value.to_string())
    }
}

impl From<concurrent_queue::PushError<String>> for ClientError {
    fn from(value: concurrent_queue::PushError<String>) -> Self {
        ClientError::QueueError(value.to_string())
    }
}

impl From<metrics_types::TypeError> for ClientError {
    fn from(value: metrics_types::TypeError) -> Self {
        ClientError::MetricsTypeError(value.to_string())
    }
}

impl From<hyper::http::Error> for ClientError {
    fn from(value: hyper::http::Error) -> Self {
        ClientError::HttpError(value.to_string())
    }
}

impl From<hyper::Error> for ClientError {
    fn from(value: hyper::Error) -> Self {
        ClientError::HttpError(value.to_string())
    }
}
