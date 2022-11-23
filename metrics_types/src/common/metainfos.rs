use super::IpAddress;

/// Meta Data generated at begining, used to fill Metrics Unit's blank
#[derive(Debug)]
pub struct MetaInfos {
    pub ip_port: IpAddress,
    pub env_name: String,
}
