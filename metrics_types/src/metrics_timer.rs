use json::JsonValue;
use serde::{Deserialize, Serialize};

use crate::alarm_wrapper::AlarmWrapper;
use crate::common::MetaInfos;
use crate::unit_jsonlog_handler::UnitJsonLogHandler;

use super::common::{IpAddress, TimeStamp};
use super::sql::SqlTable;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimerUnit {
    send_timestamp: TimeStamp,
    public_ip: IpAddress,
    category: String,
    tag: String,
    count: u64,
    max_time: u64,
    min_time: u64,
    avg_time: u64,
}

impl SqlTable for TimerUnit {
    type TypeSelf = TimerUnit;
    fn new_sql_table_opt() -> &'static str {
        r#"
        CREATE TABLE metrics_timer(
            send_timestamp INT(10) DEFAULT 0,
            public_ip VARCHAR(40) DEFAULT "",
            category VARCHAR(30) DEFAULT "",
            tag VARCHAR(100) DEFAULT "",
            count BIGINT(20) DEFAULT 0,
            max_time BIGINT(20) DEFAULT 0,
            min_time BIGINT(20) DEFAULT 0,
            avg_time BIGINT(20) DEFAULT 0,
            INDEX(category,tag,public_ip,send_timestamp)
        )ENGINE = InnoDB DEFAULT CHARSET = utf8;
        "#
    }

    fn multi_insert_table_opt() -> &'static str {
        r#"
        INSERT INTO metrics_timer ( send_timestamp, public_ip, category, tag, count, max_time, min_time, avg_time )
        VALUES
        "#
    }

    fn to_param_value_str(&self) -> String {
        format! {
            r#"({},"{}","{}","{}",{},{},{},{})"#,
            self.send_timestamp.data(),
            self.public_ip.to_string(),
            self.category.clone(),
            self.tag.clone(),
            self.count,
            self.max_time,
            self.min_time,
            self.avg_time,
        }
    }
}

impl UnitJsonLogHandler for TimerUnit {
    type UnitType = TimerUnit;

    // {"category":"xcons","tag":"network_message_dispatch","type":"timer","content":{"count":3060,"max_time":93926,"min_time":18,"avg_time":153}}
    fn handle_log(json: JsonValue, meta: MetaInfos) -> Option<AlarmWrapper<Self::UnitType>> {
        if let JsonValue::Object(obj) = json {
            let category = obj.get("category")?.as_str()?;
            let tag = obj.get("tag")?.as_str()?;
            let content = obj.get("content")?;
            let (count, max_time, min_time, avg_time) = match content {
                JsonValue::Object(content_obj) => {
                    let count = content_obj.get("count")?.as_u64()?;
                    let max_time = content_obj.get("max_time")?.as_u64()?;
                    let min_time = content_obj.get("min_time")?.as_u64()?;
                    let avg_time = content_obj.get("avg_time")?.as_u64()?;
                    (count, max_time, min_time, avg_time)
                }
                _ => {
                    return None;
                }
            };
            Some(AlarmWrapper::<TimerUnit> {
                alarm_type: crate::MetricsAlarmType::Flow,
                env: meta.env_name,
                content: TimerUnit {
                    send_timestamp: TimeStamp::now(),
                    public_ip: meta.ip_port,
                    category: category.to_string(),
                    tag: tag.to_string(),
                    count,
                    max_time,
                    min_time,
                    avg_time,
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
    #[test]
    fn test_metrics_json() {
        let timer_unit_str = r#"{"send_timestamp":"123456","public_ip":"123.456.43.21:1024","category":"some_cat","tag":"some_tag","count":10,"max_time":100,"min_time":100,"avg_time":100}"#;

        let timer_unit = serde_json::from_str::<TimerUnit>(timer_unit_str).unwrap();

        println!("{:?}", timer_unit);

        let serialized = serde_json::to_string(&timer_unit).unwrap();

        println!("{:}", serialized);
        assert_eq!(serialized, timer_unit_str);
    }
}
