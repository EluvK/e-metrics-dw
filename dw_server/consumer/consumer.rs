use std::num::NonZeroUsize;

use clap::Parser;
use futures_util::future;
use std::sync::Arc;
use tokio::sync::Mutex;

use dw_server::{consumer_backend::ConsumerBackend, redis_conn::RedisConn};
use metrics_types::{CounterUnit, FlowUnit, MetricsAlarmType, TimerUnit};
use tokio::{
    join,
    time::{sleep, Duration},
};

const FETCH_REDIS_DATA_MAX_SIZE: usize = 100;

macro_rules! HANDLE_UNIT {
    ($func:ident, $unit_type:ident, $alarm_type:expr) => {
        async fn $func(mysql_url: String) {
            let mut rc = RedisConn::new()
                .expect(format!("Create redis connection error {:?}", stringify!($func)).as_str());
            let cb = Arc::new(Mutex::new(ConsumerBackend::<$unit_type>::new(
                mysql_url,
                $alarm_type,
            )));
            loop {
                if let Ok(fetch_data) = rc.list_pop_multi(
                    &$alarm_type,
                    NonZeroUsize::new(FETCH_REDIS_DATA_MAX_SIZE).unwrap(),
                ) {
                    println!("{} size: {}", stringify!($func), fetch_data.len());
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
                    // println!("{} get empty redis", stringify!($func));
                    cb.lock().await.try_commit_all().await.unwrap();
                    sleep(Duration::from_secs(4)).await;
                }
            }
        }
    };
}

HANDLE_UNIT!(handle_counter, CounterUnit, MetricsAlarmType::Counter);
HANDLE_UNIT!(handle_timer, TimerUnit, MetricsAlarmType::Timer);
HANDLE_UNIT!(handle_flow, FlowUnit, MetricsAlarmType::Flow);

#[derive(Parser)]
struct ConsumerArgs {
    /// mysql_url
    #[clap(short = 'm', long = "mysql_url")]
    mysql_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = ConsumerArgs::parse();
    let _ = join!(
        handle_counter(args.mysql_url.clone()),
        handle_timer(args.mysql_url.clone()),
        handle_flow(args.mysql_url.clone())
    );
    Ok(())
}
