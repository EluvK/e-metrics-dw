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
