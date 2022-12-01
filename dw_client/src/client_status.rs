use std::io::Write;

use chrono::{DateTime, Utc};
use metrics_types::MetaInfos;

use crate::error::ClientError;

#[derive(Debug)]
pub(crate) struct ClientStatusInfo {
    basic_info: ClientBasicInfo,
    monitor_file_info: MonitorFileInfo,
    queue_info: QueueInfo,
    net_info: NetPacketInfo,
}

impl ClientStatusInfo {
    pub fn new(log_path: String, meta: MetaInfos) -> Self {
        ClientStatusInfo {
            basic_info: ClientBasicInfo {
                log_path,
                meta,
                start_time: Utc::now(),
            },
            monitor_file_info: MonitorFileInfo {
                current_read_pos: 0,
                file_end_pos: 0,
                total_scan_line: 0,
            },
            queue_info: QueueInfo {
                log_queue_current: 0,
                send_queue_current: 0,
            },
            net_info: NetPacketInfo {
                send_count: 0,
                success_count: 0,
            },
        }
    }

    pub fn dump(&self) -> Result<(), ClientError> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("./client_status")?;
        f.write_all(
            format!(
                concat!(
                    "=======================  Client Status  ========================\n",
                    "================================================================\n",
                    "basic info:\n{}\n",
                    "================================================================\n",
                    "file info:\n{}\n",
                    "================================================================\n",
                    "queue info:\n{}\n",
                    "net packet:\n{}\n",
                    "================================================================\n",
                ),
                self.basic_info, self.monitor_file_info, self.queue_info, self.net_info
            )
            .as_bytes(),
        )?;
        f.flush()?;
        Ok(())
    }

    pub fn update_file_info_current(&mut self, current_pos: u64) {
        self.monitor_file_info.current_read_pos = current_pos;
    }
    pub fn update_file_info_end(&mut self, end_pos: u64) {
        self.monitor_file_info.file_end_pos = end_pos;
    }
    pub fn update_file_info_line_cnt(&mut self, count: u64) {
        self.monitor_file_info.total_scan_line += count;
    }
    pub fn log_queue_current(&mut self, sz: usize) {
        self.queue_info.log_queue_current = sz;
    }
    pub fn send_queue_current(&mut self, sz: usize) {
        self.queue_info.send_queue_current = sz;
    }
    pub fn net_queue_count(&mut self, succ: bool) {
        self.net_info.send_count += 1;
        if succ {
            self.net_info.success_count += 1;
        }
    }
}

#[derive(Debug)]
struct ClientBasicInfo {
    log_path: String,
    meta: MetaInfos,
    start_time: DateTime<Utc>,
    // state_time
}

impl std::fmt::Display for ClientBasicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            concat!(
                "  * monitor log: {}\n",
                "  * meta info:   {}\n",
                "  * start at {}"
            ),
            self.log_path, self.meta, self.start_time
        )
    }
}

#[derive(Debug)]
struct MonitorFileInfo {
    current_read_pos: u64,
    file_end_pos: u64,
    total_scan_line: u64,
}

impl std::fmt::Display for MonitorFileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            concat!("  * seek pos: {}/{}\n", "  * scan lines: {}"),
            self.current_read_pos, self.file_end_pos, self.total_scan_line
        )
    }
}

#[derive(Debug)]
struct QueueInfo {
    log_queue_current: usize,
    send_queue_current: usize,
}

impl std::fmt::Display for QueueInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            concat!(
                "  * current cached queue size:\n",
                "    * log:  {}\n",
                "    * send: {}"
            ),
            self.log_queue_current, self.send_queue_current
        )
    }
}

#[derive(Debug)]
struct NetPacketInfo {
    send_count: u64,
    success_count: u64,
}

impl std::fmt::Display for NetPacketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  * net packet: {}/{}",
            self.success_count, self.send_count
        )
    }
}
