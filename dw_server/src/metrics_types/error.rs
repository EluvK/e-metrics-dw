use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("deserialize from string error: {0}")]
    DeFromStringError(String),
}
