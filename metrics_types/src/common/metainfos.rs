use std::str::FromStr;

use crate::TypeError;

use super::IpAddress;

/// Meta Data generated at begining, used to fill Metrics Unit's blank
#[derive(Debug, Clone)]
pub struct MetaInfos {
    pub server_ip_port: IpAddress,
    pub node_ip_port: IpAddress,
    pub env_name: String,
    pub(crate) server_alarm_api: String,
}

impl MetaInfos {
    pub async fn new(
        server_ip_port: String,
        self_address_use_local: bool,
        env_name: String,
    ) -> Result<MetaInfos, TypeError> {
        let node_ip_port = match self_address_use_local {
            true => IpAddress::local_ip_default_port(),
            false => IpAddress::public_ip_default_port(&server_ip_port).await?,
        };
        Ok(MetaInfos {
            server_ip_port: IpAddress::from_str(&server_ip_port)?,
            node_ip_port,
            env_name,
            server_alarm_api: String::from("http://") + &server_ip_port + "/api/alarm",
        })
    }

    pub fn alarm_api(&self) -> &str {
        &self.server_alarm_api
    }
}

impl std::fmt::Display for MetaInfos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "current node({}) alarm to server({}) with env_name: {}",
            self.node_ip_port.to_string().as_str(),
            self.server_ip_port.to_string().as_str(),
            self.env_name
        )
    }
}
