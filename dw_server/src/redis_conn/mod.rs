use std::num::NonZeroUsize;

use crate::metrics_types::MetricsAlarmType;
use redis::{Client, Commands, Connection, RedisResult};

pub struct RedisConn {
    _client: Client,
    conn: Connection,
}

impl RedisConn {
    pub fn new() -> RedisResult<Self> {
        let _client = Client::open("redis://127.0.0.1")?;
        let conn = _client.get_connection()?;
        Ok(RedisConn { _client, conn })
    }

    pub fn list_push(&mut self, key: &MetricsAlarmType, value: String) -> RedisResult<()> {
        let _ = self
            .conn
            .lpush::<String, String, ()>(key.as_redis_key(), value)?;
        Ok(())
    }

    pub fn list_pop(&mut self, key: &MetricsAlarmType) -> RedisResult<String> {
        let r = self.conn.lpop(key.as_redis_key(), None)?;
        Ok(r)
    }

    pub fn list_pop_block(&mut self, key: &MetricsAlarmType) -> RedisResult<String> {
        let r = self
            .conn
            .blpop::<String, (String, String)>(key.as_redis_key(), 0)?;
        Ok(r.1)
    }

    pub fn list_pop_multi(
        &mut self,
        key: &MetricsAlarmType,
        cnt: NonZeroUsize,
    ) -> RedisResult<Vec<String>> {
        let r = self.conn.lmpop::<String, (String, Vec<String>)>(
            1,
            key.as_redis_key(),
            redis::Direction::Left,
            cnt.get(),
        )?;
        Ok(r.1)
    }

    pub fn list_pop_block_multi(
        &mut self,
        key: &MetricsAlarmType,
        cnt: NonZeroUsize,
    ) -> RedisResult<Vec<String>> {
        let r = self.conn.blmpop::<String, (String, Vec<String>)>(
            0,
            1,
            key.as_redis_key(),
            redis::Direction::Left,
            cnt.get(),
        )?;

        Ok(r.1)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_redis() {
        let mut c = RedisConn::new().unwrap();
        c.list_push(
            &MetricsAlarmType::Counter,
            "{some metrics data}".to_string(),
        )
        .unwrap();
        let r = c.list_pop(&MetricsAlarmType::Counter).unwrap();
        assert_eq!(r, String::from("{some metrics data}"));

        c.list_push(
            &MetricsAlarmType::Flower,
            "some flow metrics data".to_string(),
        )
        .unwrap();

        let r = c.list_pop_block(&MetricsAlarmType::Flower).unwrap();
        assert_eq!(r, String::from("some flow metrics data"));

        c.list_push(
            &MetricsAlarmType::Timer,
            "some time metrics data".to_string(),
        )
        .unwrap();

        c.list_push(
            &MetricsAlarmType::Timer,
            "some time metrics data".to_string(),
        )
        .unwrap();

        let r = c
            .list_pop_multi(&MetricsAlarmType::Timer, NonZeroUsize::new(2).unwrap())
            .unwrap();
        assert_eq!(r, vec!["some time metrics data", "some time metrics data"]);

        c.list_push(
            &MetricsAlarmType::Timer,
            "some time metrics data".to_string(),
        )
        .unwrap();

        c.list_push(
            &MetricsAlarmType::Timer,
            "some time metrics data".to_string(),
        )
        .unwrap();

        c.list_push(
            &MetricsAlarmType::Timer,
            "some time metrics data".to_string(),
        )
        .unwrap();

        let r = c
            .list_pop_block_multi(&MetricsAlarmType::Timer, NonZeroUsize::new(3).unwrap())
            .unwrap();
        assert_eq!(
            r,
            vec![
                "some time metrics data",
                "some time metrics data",
                "some time metrics data"
            ]
        )
    }
}
