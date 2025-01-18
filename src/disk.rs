use std::{fs, collections::HashMap};
use axum::{routing::get, Router, response::IntoResponse};

#[derive(Debug, Default)]
struct DiskStat {
    sectors_read: u64,
    sectors_written: u64,
    read_time: u64,
    write_time: u64,
    io_time: u64,
}

fn read_diskstats() -> HashMap<String, DiskStat> {
    let content = fs::read_to_string("/proc/diskstats").expect("Failed to read /proc/diskstats");
    content.lines().filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 14 {
            return None;
        }
        let name = parts[2].to_string();
        Some((
            name,
            DiskStat {
                sectors_read: parts[5].parse().unwrap_or(0),
                sectors_written: parts[9].parse().unwrap_or(0),
                read_time: parts[6].parse().unwrap_or(0),
                write_time: parts[10].parse().unwrap_or(0),
                io_time: parts[12].parse().unwrap_or(0),
            },
        ))
    }).collect()
}

async fn disk() -> impl IntoResponse {
    let stats = read_diskstats();
    let mut result = String::new();
    for (disk, stat) in stats {
        result.push_str(&format!(
            "{}: Read={} sectors, Write={} sectors, IO Time={} ms, Read Time={} ms, Write Time={} ms\n",
            disk, stat.sectors_read, stat.sectors_written, stat.io_time, stat.read_time, stat.write_time
        ));
    }
    result
}

pub fn disk_router() -> Router {
    Router::new().route("/disk", get(disk))
}
