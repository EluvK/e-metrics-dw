use json::JsonValue;
use serde::{Deserialize, Serialize};

use crate::alarm_wrapper::AlarmWrapper;
use crate::common::MetaInfos;
use crate::unit_jsonlog_handler::UnitJsonLogHandler;

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

    fn multi_insert_table_opt() -> &'static str {
        r#"
        INSERT INTO metrics_counter ( send_timestamp, public_ip, category, tag, count, value )
        VALUES
        "#
    }

    fn to_param_value_str(&self) -> String {
        format!(
            r#"({},"{}","{}","{}",{},{})"#,
            self.send_timestamp.data(),
            self.public_ip.to_string(),
            self.category.clone(),
            self.tag.clone(),
            self.count,
            self.value
        )
    }
}

impl UnitJsonLogHandler for CounterUnit {
    type UnitType = CounterUnit;

    // {"category":"xvm","tag":"contract_manager_counter","type":"counter","content":{"count":1,"value":1}}
    fn handle_log(json: JsonValue, meta: MetaInfos) -> Option<AlarmWrapper<Self::UnitType>> {
        if let JsonValue::Object(obj) = json {
            let category = obj.get("category")?.as_str()?;
            let tag = obj.get("tag")?.as_str()?;
            let content = obj.get("content")?;
            let (count, value) = match content {
                JsonValue::Object(content_obj) => {
                    let count = content_obj.get("count")?.as_u64()?;
                    let value = content_obj.get("value")?.as_i64()?;
                    (count, value)
                }
                _ => {
                    return None;
                }
            };
            Some(AlarmWrapper::<CounterUnit> {
                alarm_type: crate::MetricsAlarmType::Counter,
                env: meta.env_name,
                content: CounterUnit {
                    send_timestamp: TimeStamp::now(),
                    public_ip: meta.ip_port,
                    category: category.to_string(),
                    tag: tag.to_string(),
                    count,
                    value,
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
        let counter_unit_str = r#"{"send_timestamp":"123456","public_ip":"123.456.43.21:1024","category":"some_cat","tag":"some_tag","count":10,"value":100}"#;

        let counter_unit = serde_json::from_str::<CounterUnit>(counter_unit_str).unwrap();

        println!("{:?}", counter_unit);

        let serialized = serde_json::to_string(&counter_unit).unwrap();

        println!("{:}", serialized);
        assert_eq!(serialized, counter_unit_str);
    }
}
