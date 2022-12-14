use std::{str::FromStr, time::SystemTime};

use serde::{Deserialize, Serialize};

use crate::TypeError;

#[cfg(feature = "fake_data")]
use fake::{Dummy, Fake};

#[derive(Debug)]
#[cfg_attr(feature = "fake_data", derive(Dummy))]
pub(crate) struct TimeStamp {
    #[cfg_attr(feature = "fake_data", dummy(faker = "1670000000..1680000000"))]
    ts: u32,
}

impl Serialize for TimeStamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TimeStamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;
        TimeStamp::from_str(&buf).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

impl ToString for TimeStamp {
    fn to_string(&self) -> String {
        self.ts.to_string()
    }
}

impl FromStr for TimeStamp {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ts = s
            .parse::<u32>()
            .map_err(|err| TypeError::DeFromStringError(err.to_string()))?;
        Ok(TimeStamp { ts })
    }
}

impl TimeStamp {
    pub(crate) fn now() -> Self {
        TimeStamp {
            ts: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
        }
    }
    pub(crate) fn data(&self) -> u32 {
        self.ts
    }
}
