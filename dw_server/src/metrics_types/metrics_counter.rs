use mysql_async::params;
use serde::{Deserialize, Serialize};

use super::common::{IpAddress, TimeStamp};
use super::sql::SqlTable;

#[derive(Debug, Deserialize, Serialize)]
pub struct CounterUnit {
    send_timestamp: TimeStamp,
    public_ip: IpAddress,
    category: String,
    tag: String,
    count: u64,
    value: i64,
}

impl CounterUnit {
    #[cfg(test)]
    pub fn rand() -> Self {
        CounterUnit {
            send_timestamp: TimeStamp::rand(),
            public_ip: IpAddress::rand(),
            category: String::from("cat"),
            tag: String::from("tag"),
            count: 10,
            value: 100,
        }
    }
}

impl SqlTable for CounterUnit {
    type TypeSelf = CounterUnit;
    fn new_sql_table_opt() -> &'static str {
        r#"
        CREATE TABLE metrics_counter(
            send_timestamp INT(10) DEFAULT 0,
            public_ip VARCHAR(40) DEFAULT "",
            category VARCHAR(30) DEFAULT "",
            tag VARCHAR(100) DEFAULT "",
            count BIGINT(20) DEFAULT 0,
            value BIGINT(20) DEFAULT 0,
            INDEX(category,tag,public_ip,send_timestamp)
        )ENGINE = InnoDB DEFAULT CHARSET = utf8;
        "#
    }

    fn insert_table_opt() -> &'static str {
        r#"
        INSERT INTO metrics_counter ( send_timestamp, public_ip, category, tag, count, value )
        VALUES (:send_timestamp, :public_ip, :category, :tag, :count, :value )
        "#
    }

    fn to_params(&self) -> mysql_async::Params {
        params! {
            "send_timestamp" => self.send_timestamp.data(),
            "public_ip" => self.public_ip.to_string(),
            "category" => self.category.clone(),
            "tag" => self.tag.clone(),
            "count" => self.count,
            "value" => self.value,
        }
    }
}
