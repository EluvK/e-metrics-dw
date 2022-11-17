use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::metrics_types::TypeError;

#[derive(Debug)]
pub(crate) struct IpAddress {
    ip: String,
    port: usize,
}

impl Serialize for IpAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for IpAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;
        IpAddress::from_str(&buf).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

impl ToString for IpAddress {
    fn to_string(&self) -> String {
        self.ip.clone() + ":" + &self.port.to_string()
    }
}

impl FromStr for IpAddress {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s.split_once(':').ok_or(TypeError::DeFromStringError(
            "string split port error".into(),
        ))?;
        Ok(IpAddress {
            ip: String::from(r.0),
            port: r
                .1
                .parse::<usize>()
                .map_err(|err| TypeError::DeFromStringError(err.to_string()))?,
        })
    }
}

impl IpAddress {
    #[cfg(test)]
    pub(crate) fn rand() -> IpAddress {
        IpAddress {
            ip: String::from("11.11.11.11"),
            port: 11,
        }
    }
}
