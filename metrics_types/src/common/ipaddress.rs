use std::str::FromStr;

use hyper::{Body, Client, Request, StatusCode};
use lazy_static::lazy_static;
use local_ip_address::linux::local_ip;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::TypeError;

#[cfg(feature = "fake_data")]
use fake::faker::internet::en::IP;
#[cfg(feature = "fake_data")]
use fake::{Dummy, Fake};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fake_data", derive(Dummy))]
pub struct IpAddress {
    #[cfg_attr(feature = "fake_data", dummy(faker = "IP()"))]
    ip: String,
    #[cfg_attr(feature = "fake_data", dummy(faker = "1000..2000"))]
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
        let r = s
            .split_once(':')
            .ok_or(TypeError::DeFromStringError("ipaddress string split port error".into()))?;
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$"#).unwrap();
        }
        if !RE.is_match(r.0) {
            return Err(TypeError::DeFromStringError("ipaddress string ip format error".into()));
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

    pub async fn public_ip_default_port(server_ip_port: &String) -> Result<IpAddress, TypeError> {
        let req = Request::builder()
            .method("GET")
            .uri(String::from("http://") + server_ip_port + "/api/ip")
            .body(Body::empty())
            .map_err(|e| TypeError::CustomError(e.to_string()))?;

        let mut r = Client::new()
            .request(req)
            .await
            .map_err(|e| TypeError::CustomError(e.to_string()))?;

        match r.status() {
            StatusCode::OK => {
                let body = hyper::body::to_bytes(r.body_mut())
                    .await
                    .map_err(|e| TypeError::CustomError(e.to_string()))?;
                let ip =
                    String::from_utf8(body.into_iter().collect()).map_err(|e| TypeError::CustomError(e.to_string()))?;
                Ok(IpAddress { ip, port: 9000 })
            }
            _ => Err(TypeError::CustomError("query public ip failed".into())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_local_ip() {
        let ip = IpAddress::local_ip_default_port();
        println!("{:?}", ip);
    }

    async fn get_public_ip() -> Result<IpAddress, TypeError> {
        let server_ip_port = String::from("127.0.0.1:3000");
        IpAddress::public_ip_default_port(&server_ip_port).await
    }

    #[test]
    fn test_public_ip() {
        let ip = tokio_test::block_on(get_public_ip()).unwrap();
        println!("{:?}", ip);
    }
}
