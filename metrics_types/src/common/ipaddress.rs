use std::str::FromStr;

use lazy_static::lazy_static;
use local_ip_address::linux::local_ip;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::TypeError;

#[derive(Debug, Clone)]
pub struct IpAddress {
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
            "ipaddress string split port error".into(),
        ))?;
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$"#).unwrap();
        }
        if !RE.is_match(r.0) {
            return Err(TypeError::DeFromStringError(
                "ipaddress string ip format error".into(),
            ));
        }
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
    pub fn local_ip_default_port() -> IpAddress {
        let local_ip = local_ip().unwrap();
        IpAddress {
            ip: local_ip.to_string(),
            port: 9000,
        }
    }

    pub fn public_ip_default_port(public_ip: String) -> IpAddress {
        IpAddress {
            ip: public_ip,
            port: 9000,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ip() {
        let ip = IpAddress::local_ip_default_port();
        println!("{:?}", ip);
    }
}
