#![allow(unused)]
use std::collections::HashMap;

use mysql_async::Result;
use serde::de::Deserialize;

use std::time::{Duration, Instant};

use crate::metrics_types::alarm_wrapper::AlarmWrapper;
use crate::metrics_types::{sql::SqlTable, MetricsAlarmType};
use crate::mysql_conn::MysqlDBConn;

/// Each AlarmType-DB have one Inner type.
/// reserved for futher function.
#[derive(Debug)]
struct ConsumerBackendInner<UnitType> {
    // _db_name: String,
    cache_data: Vec<UnitType>,
    mysql_conn: MysqlDBConn,
    commit_time: Instant,
}

const CACHE_DATA_MUST_COMMIT_LEN: usize = 1000;

impl<UnitType> ConsumerBackendInner<UnitType>
where
    UnitType: SqlTable + Send + Sync,
{
    async fn new(db_name: &String) -> Result<Self> {
        Ok(ConsumerBackendInner {
            cache_data: Vec::new(),
            mysql_conn: MysqlDBConn::new(String::from(crate::config::CONSUMER_MYSQL_URL), db_name)
                .await?,
            commit_time: Instant::now(),
        })
    }

    async fn cache(&mut self, data: UnitType) -> Result<()> {
        self.cache_data.push(data);
        if self.cache_data.len() > CACHE_DATA_MUST_COMMIT_LEN {
            self.try_commit().await?;
        }
        Ok(())
    }

    async fn try_commit(&mut self) -> Result<()> {
        if self.cache_data.len() > CACHE_DATA_MUST_COMMIT_LEN {
            let r = self
                .cache_data
                .drain(0..CACHE_DATA_MUST_COMMIT_LEN)
                .collect();
            self.mysql_conn.insert(r).await?;
            self.commit_time = Instant::now();
        } else if self.commit_time.elapsed() > Duration::from_secs(3) && !self.cache_data.is_empty()
        {
            let r = self.cache_data.drain(..).collect();
            self.mysql_conn.insert(r).await?;
            self.commit_time = Instant::now();
        }
        Ok(())
    }
}

/// Each `ConsumerBackend` server for specifical MetricsUnit, but all database concurrently.
/// So the number of `ConsumerBackend` object should be equal to the number of MetricsAlarmType.
#[derive(Debug)]
pub struct ConsumerBackend<UnitType> {
    inner_cache: HashMap<String, ConsumerBackendInner<UnitType>>,
}

impl<'de, UnitType> ConsumerBackend<UnitType>
where
    UnitType: Deserialize<'de> + SqlTable + Send + Sync,
{
    pub fn new(_alarm_type: MetricsAlarmType) -> Self {
        ConsumerBackend {
            inner_cache: HashMap::new(),
        }
    }

    pub async fn cache(&mut self, data_str: &'de str) -> Result<()> {
        // println!("{}", data_str);
        let wrapped_unit: AlarmWrapper<UnitType> = serde_json::from_str(data_str).map_err(|e| {
            println!("{}", e);
            mysql_async::Error::Other(Box::from("wrapped unit deserialize error"))
        })?;
        let db_name = wrapped_unit.env;
        if !self.inner_cache.contains_key(&db_name) {
            self.inner_cache
                .insert(db_name.clone(), ConsumerBackendInner::new(&db_name).await?);
        }
        self.inner_cache
            .get_mut(&db_name)
            .unwrap()
            .cache(wrapped_unit.content)
            .await
    }

    pub async fn try_commit_all(&mut self) -> Result<()> {
        for (db, cb) in self.inner_cache.iter_mut() {
            cb.try_commit().await?;
        }

        Ok(())
    }
}
