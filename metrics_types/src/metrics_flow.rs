use json::JsonValue;
use serde::{Deserialize, Serialize};

use crate::alarm_wrapper::AlarmWrapper;
use crate::common::MetaInfos;
use crate::unit_jsonlog_handler::UnitJsonLogHandler;

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

impl UnitJsonLogHandler for FlowUnit {
    type UnitType = FlowUnit;

    fn handle_log(json: JsonValue, meta: &MetaInfos) -> Option<AlarmWrapper<Self::UnitType>> {
        if let JsonValue::Object(obj) = json {
            let category = obj.get("category")?.as_str()?;
            let tag = obj.get("tag")?.as_str()?;
            let content = obj.get("content")?;
            let (count, max_flow, min_flow, sum_flow, avg_flow, tps_flow, tps) = match content {
                JsonValue::Object(content_obj) => {
                    let count = content_obj.get("count")?.as_u64()?;
                    let max_flow = content_obj.get("max_flow")?.as_i64()?;
                    let min_flow = content_obj.get("min_flow")?.as_i64()?;
                    let sum_flow = content_obj.get("sum_flow")?.as_i64()?;
                    let avg_flow = content_obj.get("avg_flow")?.as_i64()?;
                    let tps_flow = content_obj.get("tps_flow")?.as_i64()?;
                    let tps = content_obj.get("tps")?.as_str()?.parse::<f64>().ok()?;
                    (count, max_flow, min_flow, sum_flow, avg_flow, tps_flow, tps)
                }
                _ => {
                    return None;
                }
            };
            Some(AlarmWrapper::<FlowUnit> {
                alarm_type: crate::MetricsAlarmType::Flow,
                env: meta.env_name.clone(),
                content: FlowUnit {
                    send_timestamp: TimeStamp::now(),
                    public_ip: meta.node_ip_port.clone(),
                    category: category.to_string(),
                    tag: tag.to_string(),
                    count,
                    max_flow,
                    min_flow,
                    sum_flow,
                    avg_flow,
                    tps_flow,
                    tps,
                },
            })
        } else {
            None
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn test_metrics_json() {
        let flow_unit_str = r#"{"send_timestamp":"123456","public_ip":"123.12.34.21:1024","category":"some_cat","tag":"some_tag","count":10,"max_flow":1000,"min_flow":10,"sum_flow":1001123,"avg_flow":133,"tps_flow":1093,"tps":100.212}"#;

        let flow_unit = serde_json::from_str::<FlowUnit>(flow_unit_str).unwrap();

        println!("{:?}", flow_unit);

        let serialized = serde_json::to_string(&flow_unit).unwrap();

        println!("{:}", serialized);
        assert_eq!(serialized, flow_unit_str);
    }

    #[test]
    fn test_metrics_log() {
        let json_object = json::parse(
            r#"{"category":"vhost","tag":"handle_data_ready_called","type":"flow","content":{"count":92,"max_flow":10,"min_flow":1,"sum_flow":131,"avg_flow":1,"tps_flow":131,"tps":"1.39"}}"#,
        ).unwrap();

        println!("{:?}", json_object);

        let meta = MetaInfos {
            node_ip_port: IpAddress::local_ip_default_port(),
            server_ip_port: IpAddress::from_str("127.0.0.1:3000").unwrap(),
            env_name: String::from("test_env_name"),
        };

        let result = FlowUnit::handle_log(json_object, &meta);

        println!("{:?}", result);
        assert_eq!(result.is_some(), true);
    }
}
