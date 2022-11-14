use std::num::NonZeroUsize;

use futures_util::future;
use std::sync::Arc;
use tokio::sync::Mutex;

use dw_server::{
    consumer_backend::ConsumerBackend,
    metrics_types::{CounterUnit, MetricsAlarmType, TimerUnit},
    redis_conn::RedisConn,
};
use tokio::{
    join,
    time::{sleep, Duration},
};

const FETCH_REDIS_DATA_MAX_SIZE: usize = 100;

async fn handle_metrics_counter() {
    let mut rc = RedisConn::new().expect("Create redis connnection error");

    let cb = Arc::new(Mutex::new(ConsumerBackend::<CounterUnit>::new(
        MetricsAlarmType::Counter,
    )));

    loop {
        if let Ok(fetch_data) = rc.list_pop_multi(
            &MetricsAlarmType::Counter,
            NonZeroUsize::new(FETCH_REDIS_DATA_MAX_SIZE).unwrap(),
        ) {
            println!("handle counter size: {}", fetch_data.len());
            // for str in fetch_data {
            //     cb.lock().await.cache(&str).await.unwrap_or(());
            // }

            let tasks: Vec<_> = fetch_data
                .iter()
                .map(|d| {
                    let data = d.clone();
                    let cb = cb.clone();
                    tokio::spawn(async move {
                        cb.lock().await.cache(&data).await.unwrap_or(()); // TODO unwrap?
                    })
                })
                .collect();
            future::join_all(tasks).await;
        } else {
            println!("empty counter");
            cb.lock().await.try_commit_all().await.unwrap();
            sleep(Duration::from_secs(2)).await;
        }
    }
}
async fn handle_metrics_timer() {
    let mut rc = RedisConn::new().expect("Create redis connnection error");

    let cb = Arc::new(Mutex::new(ConsumerBackend::<TimerUnit>::new(
        MetricsAlarmType::Timer,
    )));

    loop {
        if let Ok(fetch_data) = rc.list_pop_multi(
            &MetricsAlarmType::Timer,
            NonZeroUsize::new(FETCH_REDIS_DATA_MAX_SIZE).unwrap(),
        ) {
            println!("handle timer size: {}", fetch_data.len());
            let tasks: Vec<_> = fetch_data
                .iter()
                .map(|d| {
                    let data = d.clone();
                    let cb = cb.clone();
                    tokio::spawn(async move {
                        cb.lock().await.cache(&data).await.unwrap_or(()); // TODO unwrap?
                    })
                })
                .collect();
            future::join_all(tasks).await;
        } else {
            println!("empty timer");
            cb.lock().await.try_commit_all().await.unwrap();
            sleep(Duration::from_secs(2)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = join!(handle_metrics_counter(), handle_metrics_timer());
    Ok(())
}
