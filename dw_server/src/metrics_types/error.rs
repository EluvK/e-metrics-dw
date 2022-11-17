use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("deserialize from string error: {0}")]
    DeFromStringError(String),

    #[error("metrics alarm type invalid")]
    MetricsAlarmTypeInvalid,
}
