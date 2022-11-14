use serde::{Deserialize, Serialize};

use crate::metrics_types::MetricsAlarmType;

#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmWrapper<UnitType> {
    pub alarm_type: MetricsAlarmType,
    pub env: String,
    pub content: UnitType,
}
