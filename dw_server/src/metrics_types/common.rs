use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct IpAddress {
    ip: String,
    port: usize,
}

impl IpAddress {
    pub(crate) fn to_string(&self) -> String {
        self.ip.clone() + &self.port.to_string()
    }

    #[cfg(test)]
    pub(crate) fn rand() -> IpAddress {
        IpAddress {
            ip: String::from("11.11.11.11"),
            port: 11,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimeStamp {
    ts: u32,
}

impl TimeStamp {
    pub(crate) fn data(&self) -> u32 {
        self.ts
    }

    #[cfg(test)]
    pub(crate) fn rand() -> TimeStamp {
        TimeStamp { ts: 123 }
    }
}
