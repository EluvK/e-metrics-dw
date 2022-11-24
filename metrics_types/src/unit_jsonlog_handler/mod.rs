use crate::alarm_wrapper::AlarmWrapper;
use crate::common::MetaInfos;

use json::JsonValue;

// handle log's line data to metrics alarm json format corresponding data.
pub trait UnitJsonLogHandler {
    type UnitType;

    fn handle_log(json: JsonValue, meta: &MetaInfos) -> Option<AlarmWrapper<Self::UnitType>>;
}
