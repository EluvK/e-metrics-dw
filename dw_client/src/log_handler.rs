use crate::error::ClientError;
use concurrent_queue::ConcurrentQueue;
use hyper::{Body, Client, Method, Request};
use lazy_static::lazy_static;
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    io::{AsyncSeekExt, SeekFrom},
    join,
};

use regex::Regex;

use metrics_types::{
    unit_jsonlog_handler::UnitJsonLogHandler, CounterUnit, FlowUnit, MetaInfos, MetricsAlarmType,
    TimerUnit,
};

pub struct LogHandler {
    log_path: String,
    _env_name: String,
    meta: MetaInfos,
}

impl LogHandler {
    pub fn new(
        self_ip_port: String,
        log_path: String,
        env_name: String,
    ) -> Result<Self, ClientError> {
        if !std::path::Path::exists(std::path::Path::new(&log_path)) {
            return Err(ClientError::FileIOError(String::from("log file not exist")));
        }
        Ok(Self {
            log_path,
            _env_name: env_name.clone(),
            meta: MetaInfos::new(self_ip_port, env_name)?,
        })
    }

    pub async fn start(&self) -> Result<!, ClientError> {
        let metrics_log_queue = Arc::new(ConcurrentQueue::<String>::unbounded());
        let metrics_send_queue = Arc::new(ConcurrentQueue::<String>::unbounded());

        let r = join!(
            self.monitor_file(metrics_log_queue.clone()),
            self.handle_metrics_log(metrics_log_queue.clone(), metrics_send_queue.clone()),
            self.send_alarm(metrics_send_queue.clone())
        );
        r.0.and(r.1).and(r.2)
    }

    async fn monitor_file(
        &self,
        metrics_log_queue: Arc<ConcurrentQueue<String>>,
    ) -> Result<!, ClientError> {
        let file = File::open(&self.log_path).await?;
        let mut buf_reader = BufReader::new(file);

        let mut last_read_pos: u64 = 0;
        let mut file_might_stop_count: usize = 0;
        loop {
            let mut content = String::new();
            let _ = buf_reader.seek(SeekFrom::Start(last_read_pos)).await?;
            buf_reader.read_line(&mut content).await?;
            let new_pos = buf_reader.stream_position().await?;
            println!("new_pos: {}", new_pos);
            if new_pos > last_read_pos {
                // read new content;
                // println!("Insert : {}", content);
                metrics_log_queue.push(content)?;
                last_read_pos = new_pos;
                file_might_stop_count = 0;
            } else {
                // log file might stop logging, or restart from begining.
                tokio::time::sleep(Duration::from_secs(1)).await;
                file_might_stop_count += 1;
                if file_might_stop_count >= 4 {
                    let file_end_pos = buf_reader.seek(SeekFrom::End(0)).await?;
                    println!("file_end_pos:{}", file_end_pos);
                    file_might_stop_count = 0;
                    if file_end_pos < new_pos {
                        // re-begin read file from begining.
                        last_read_pos = 0;
                    }
                }
            }
        }
    }

    async fn handle_metrics_log(
        &self,
        metrics_log_queue: Arc<ConcurrentQueue<String>>,
        metrics_send_queue: Arc<ConcurrentQueue<String>>,
    ) -> Result<!, ClientError> {
        loop {
            let mut cnt = 0;
            while cnt < 10 && metrics_log_queue.len() < 10 {
                cnt = cnt + 1;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            // 1s timeout or len > 10
            match metrics_log_queue.pop() {
                Ok(log) => {
                    println!("Got : {}", log);
                    if let Some(r) = self.handler_metrics(log) {
                        metrics_send_queue.push(r)?;
                    }
                    let mut continuous_pop_cnt = 1;
                    while !metrics_log_queue.is_empty() && continuous_pop_cnt < 10 {
                        match metrics_log_queue.pop() {
                            Ok(log) => {
                                println!("Got : {}", log);
                                if let Some(r) = self.handler_metrics(log) {
                                    metrics_send_queue.push(r)?;
                                }
                            }
                            Err(_) => {
                                break;
                            }
                        }
                        continuous_pop_cnt = continuous_pop_cnt + 1;
                    }
                }
                Err(concurrent_queue::PopError::Empty) => {
                    println!("Empty queue");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Err(concurrent_queue::PopError::Closed) => {
                    return Err(ClientError::QueueError(
                        concurrent_queue::PopError::Closed.to_string(),
                    ));
                }
            }
        }
    }

    async fn send_alarm(
        &self,
        metrics_send_queue: Arc<ConcurrentQueue<String>>,
    ) -> Result<!, ClientError> {
        loop {
            let mut cnt = 0;
            while cnt < 10 && metrics_send_queue.len() < 10 {
                cnt = cnt + 1;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            // 1s timeout or len > 10

            match metrics_send_queue.pop() {
                Ok(send_data) => {
                    let mut send_data_vec = Vec::new();
                    send_data_vec.push(send_data);
                    // println!("send queue got send_data : {}", send_data);
                    let mut continuous_pop_cnt = 1;
                    while !metrics_send_queue.is_empty() && continuous_pop_cnt < 10 {
                        match metrics_send_queue.pop() {
                            Ok(send_data) => {
                                // println!("Got : {}", send_data);
                                send_data_vec.push(send_data);
                            }
                            Err(_) => {
                                break;
                            }
                        }
                        continuous_pop_cnt = continuous_pop_cnt + 1;
                    }
                    let send_combined = String::from("[") + &send_data_vec.join(",") + "]";
                    self.do_batch_send_alarm(send_combined).await?;
                }
                Err(concurrent_queue::PopError::Empty) => {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Err(concurrent_queue::PopError::Closed) => {
                    return Err(ClientError::QueueError(
                        concurrent_queue::PopError::Closed.to_string(),
                    ))
                }
            }
        }
    }

    async fn do_batch_send_alarm(&self, data: String) -> Result<(), ClientError> {
        println!("do send: {}", data);
        let req = Request::builder()
            .method(Method::POST)
            .uri("http://127.0.0.1:3000/api/alarm") // TODO make uri para
            .header("content-type", "application/json")
            .body(Body::from(data))?;
        let resp = Client::new().request(req).await?;
        println!("resp: {:?}", resp);
        Ok(())
    }

    fn handler_metrics(&self, log: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"\[metrics\](?P<fulllog>\{.*"type":"(?P<type>[a-zA-Z_]*)".*\})"#)
                    .unwrap();
        }
        let match_result = RE.captures(&log).and_then(|cap| {
            let fulllog = cap.name("fulllog");
            let r#type = cap.name("type");
            if None == fulllog || None == r#type {
                None
            } else {
                Some((fulllog.unwrap(), r#type.unwrap()))
            }
        });
        let mut result = None;
        if let Some((fulllog, r#type)) = match_result {
            let type_str = r#type.as_str();
            let fulllog_str = fulllog.as_str();
            if let Ok(json_value) = json::parse(fulllog_str) {
                if let Ok(alarm_type) = MetricsAlarmType::from_str(type_str) {
                    match alarm_type {
                        MetricsAlarmType::Counter => {
                            let wrapper_unit = CounterUnit::handle_log(json_value, &self.meta)?;
                            result = serde_json::to_string(&wrapper_unit).ok();
                        }
                        MetricsAlarmType::Timer => {
                            let wrapper_unit = TimerUnit::handle_log(json_value, &self.meta)?;
                            result = serde_json::to_string(&wrapper_unit).ok();
                        }
                        MetricsAlarmType::Flow => {
                            let wrapper_unit = FlowUnit::handle_log(json_value, &self.meta)?;
                            result = serde_json::to_string(&wrapper_unit).ok();
                        }
                        MetricsAlarmType::Invalid => {} // don't use `_` here. So will force add missing enum case when add more types
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {

    use super::*;

    async fn do_send_test() {
        let data = r#"[{"alarm_type":"flow","env":"test_db","content":{"send_timestamp":"1669269373","public_ip":"127.0.0.1:9000","category":"vhost","tag":"handle_data_ready_called","count":1983,"max_flow":10,"min_flow":1,"sum_flow":2463,"avg_flow":1,"tps_flow":1620,"tps":8.99}},{"alarm_type":"flow","env":"test_db","content":{"send_timestamp":"1669269373","public_ip":"127.0.0.1:9000","category":"vhost","tag":"handle_data_ready_called","count":3340,"max_flow":10,"min_flow":1,"sum_flow":4146,"avg_flow":1,"tps_flow":1683,"tps":9.34}},{"alarm_type":"flow","env":"test_db","content":{"send_timestamp":"1669269373","public_ip":"127.0.0.1:9000","category":"xcons","tag":"network_message_dispatch","count":3060,"max_time":93926,"min_time":18,"avg_time":153}},{"alarm_type":"flow","env":"test_db","content":{"send_timestamp":"1669269373","public_ip":"127.0.0.1:9000","category":"xsync","tag":"network_message_dispatch","count":2630,"max_time":45861,"min_time":13,"avg_time":118}},{"alarm_type":"counter","env":"test_db","content":{"send_timestamp":"1669269373","public_ip":"127.0.0.1:9000","category":"xvm","tag":"contract_manager_counter","count":1,"value":1}},{"alarm_type":"counter","env":"test_db","content":{"send_timestamp":"1669269373","public_ip":"127.0.0.1:9000","category":"xvm","tag":"contract_role_context_counter","count":44,"value":16}},{"alarm_type":"flow","env":"test_db","content":{"send_timestamp":"1669269373","public_ip":"127.0.0.1:9000","category":"vhost","tag":"handle_data_ready_called","count":1983,"max_flow":10,"min_flow":1,"sum_flow":2463,"avg_flow":1,"tps_flow":1620,"tps":8.99}},{"alarm_type":"flow","env":"test_db","content":{"send_timestamp":"1669269373","public_ip":"127.0.0.1:9000","category":"vhost","tag":"handle_data_ready_called","count":3340,"max_flow":10,"min_flow":1,"sum_flow":4146,"avg_flow":1,"tps_flow":1683,"tps":9.34}},{"alarm_type":"flow","env":"test_db","content":{"send_timestamp":"1669269373","public_ip":"127.0.0.1:9000","category":"xcons","tag":"network_message_dispatch","count":3060,"max_time":93926,"min_time":18,"avg_time":153}}]"#;
        let req = Request::builder()
            .method(Method::POST)
            .uri("http://127.0.0.1:3000/api/alarm")
            .header("content-type", "application/json")
            .body(Body::from(data))
            .unwrap();
        let resp = Client::new().request(req).await.unwrap();
        println!("resp: {:?}", resp);
    }

    #[test]
    fn test_send() {
        tokio_test::block_on(do_send_test());
    }
}
