use std::str::FromStr;

use crate::TypeError;

use super::IpAddress;

/// Meta Data generated at begining, used to fill Metrics Unit's blank
#[derive(Debug)]
pub struct MetaInfos {
    pub ip_port: IpAddress,
    pub env_name: String,
}

impl MetaInfos {
    pub fn new(ip_port: String, env_name: String) -> Result<MetaInfos, TypeError> {
        Ok(MetaInfos {
            ip_port: IpAddress::from_str(&ip_port)?,
            env_name,
        })
    }
}
