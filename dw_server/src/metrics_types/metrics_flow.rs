use mysql_async::params;
use serde::{Deserialize, Serialize};

use super::common::{IpAddress, TimeStamp};
use super::sql::SqlTable;

#[derive(Debug, Deserialize, Serialize)]
pub struct FlowUnit {
    send_timestamp: TimeStamp,
    public_ip: IpAddress,
    category: String,
    tag: String,
    count: u64,
    max_flow: i64,
    min_flow: i64,
    sum_flow: i64,
    avg_flow: i64,
    tps_flow: i64,
    tps: f64,
}

impl SqlTable for FlowUnit {
    type TypeSelf = FlowUnit;
    fn new_sql_table_opt() -> &'static str {
        r#"
        CREATE TABLE metrics_flow(
            send_timestamp INT(10) DEFAULT 0,
            public_ip VARCHAR(40) DEFAULT "",
            category VARCHAR(30) DEFAULT "",
            tag VARCHAR(100) DEFAULT "",
            count BIGINT(20) DEFAULT 0,
            max_flow BIGINT(20) DEFAULT 0,
            min_flow BIGINT(20) DEFAULT 0,
            sum_flow BIGINT(20) DEFAULT 0,
            avg_flow BIGINT(20) DEFAULT 0,
            tps_flow BIGINT(20) DEFAULT 0,
            tps DOUBLE DEFAULT 0.00,
            INDEX(category,tag,public_ip,send_timestamp)
        )ENGINE = InnoDB DEFAULT CHARSET = utf8;
        "#
    }

    fn insert_table_opt() -> &'static str {
        r#"
        INSERT INTO metrics_flow ( send_timestamp, public_ip, category, tag, count, max_flow, min_flow, sum_flow, avg_flow, tps_flow, tps )
        VALUES (:send_timestamp, :public_ip, :category, :tag, :count, :max_flow, :min_flow, :sum_flow, :avg_flow, :tps_flow, :tps )
        "#
    }

    fn to_params(&self) -> mysql_async::Params {
        params! {
            "send_timestamp" => self.send_timestamp.data(),
            "public_ip" => self.public_ip.to_string(),
            "category" => self.category.clone(),
            "tag" => self.tag.clone(),
            "count" => self.count,
            "max_flow" => self.max_flow,
            "min_flow" => self.min_flow,
            "sum_flow" => self.sum_flow,
            "avg_flow" => self.avg_flow,
            "tps_flow" => self.tps_flow,
            "tps" => self.tps,
        }
    }
}
