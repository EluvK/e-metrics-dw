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

    fn multi_insert_table_opt() -> &'static str {
        r#"
        INSERT INTO metrics_flow ( send_timestamp, public_ip, category, tag, count, max_flow, min_flow, sum_flow, avg_flow, tps_flow, tps )
        VALUES
        "#
    }

    fn to_param_value_str(&self) -> String {
        format! {
            r#"({},"{}","{}","{}",{},{},{},{},{},{},{})"#,
            self.send_timestamp.data(),
            self.public_ip.to_string(),
            self.category.clone(),
            self.tag.clone(),
            self.count,
            self.max_flow,
            self.min_flow,
            self.sum_flow,
            self.avg_flow,
            self.tps_flow,
            self.tps,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_metrics_json() {
        let flow_unit_str = r#"{"send_timestamp":"123456","public_ip":"123.456.43.21:1024","category":"some_cat","tag":"some_tag","count":10,"max_flow":1000,"min_flow":10,"sum_flow":1001123,"avg_flow":133,"tps_flow":1093,"tps":100.212}"#;

        let flow_unit = serde_json::from_str::<FlowUnit>(flow_unit_str).unwrap();

        println!("{:?}", flow_unit);

        let serialized = serde_json::to_string(&flow_unit).unwrap();

        println!("{:}", serialized);
        assert_eq!(serialized, flow_unit_str);
    }
}
