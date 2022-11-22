use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::TypeError;

#[derive(Debug)]
pub enum MetricsAlarmType {
    Invalid,
    Counter,
    Timer,
    Flow,
}

impl Serialize for MetricsAlarmType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MetricsAlarmType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;
        MetricsAlarmType::from_str(&buf).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

impl FromStr for MetricsAlarmType {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "counter" => Ok(MetricsAlarmType::Counter),
            "timer" => Ok(MetricsAlarmType::Timer),
            "flow" => Ok(MetricsAlarmType::Flow),
            _ => Err(TypeError::MetricsAlarmTypeInvalid),
        }
    }
}

impl ToString for MetricsAlarmType {
    fn to_string(&self) -> String {
        match self {
            MetricsAlarmType::Counter => String::from("counter"),
            MetricsAlarmType::Timer => String::from("timer"),
            MetricsAlarmType::Flow => String::from("flow"),
            MetricsAlarmType::Invalid => String::from("invalid"),
        }
    }
}

impl MetricsAlarmType {
    #[inline]
    pub fn as_redis_key(&self) -> String {
        self.to_string()
    }
}
