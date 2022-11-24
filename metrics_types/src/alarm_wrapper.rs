use serde::{Deserialize, Serialize};

use crate::MetricsAlarmType;

/// #### AlarmWrapper
///
/// Wrap `UnitType` as `"content"`, with `"alarm_type"` && `"env"`
///
/// Used in Net Transport、Proxy and ConsumerBackend.
///
/// Example:
/// A `UnitType` counter: `CounterUnit`:
///
/// ``` json
/// {
///    "send_timestamp": "123456",
///    "public_ip": "123.456.43.21:1024",
///    "category": "some_cat",
///    "tag": "some_tag",
///    "count": 10,
///    "value": 100
/// }
/// ```
///
/// Then `AlarmWrapper<UnitType>` :
///
/// ``` json
/// {
///     "alarm_type": "counter",
///     "env": "db_name",
///     "content": {
///         "send_timestamp": "123456",
///         "public_ip": "123.456.43.21:1024",
///         "category": "some_cat",
///         "tag": "some_tag",
///         "count": 10,
///         "value": 100
///     }
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmWrapper<UnitType> {
    pub alarm_type: MetricsAlarmType,
    pub env: String,
    pub content: UnitType,
}
