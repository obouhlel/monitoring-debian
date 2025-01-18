use std::fs;
use axum::{routing::get, Router, response::IntoResponse};

#[derive(Debug, Default)]
struct MemoryInfo {
    total: u64,
    free: u64,
    used: u64,
    buffers: u64,
    cached: u64,
    swap_total: u64,
    swap_free: u64,
    swap_used: u64,
}

fn read_meminfo() -> MemoryInfo {
    let content = fs::read_to_string("/proc/meminfo").expect("Failed to read /proc/meminfo");
    let mut meminfo = MemoryInfo::default();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        match parts[0] {
            "MemTotal:" => meminfo.total = parts[1].parse().unwrap_or(0),
            "MemFree:" => meminfo.free = parts[1].parse().unwrap_or(0),
            "Buffers:" => meminfo.buffers = parts[1].parse().unwrap_or(0),
            "Cached:" => meminfo.cached = parts[1].parse().unwrap_or(0),
            "SwapTotal:" => meminfo.swap_total = parts[1].parse().unwrap_or(0),
            "SwapFree:" => meminfo.swap_free = parts[1].parse().unwrap_or(0),
            _ => {}
        }
    }
    meminfo.used = meminfo.total.saturating_sub(meminfo.free + meminfo.buffers + meminfo.cached);
    meminfo.swap_used = meminfo.swap_total.saturating_sub(meminfo.swap_free);
    meminfo
}

async fn ram() -> impl IntoResponse {
    let meminfo = read_meminfo();
    format!(
        "RAM: {:.2}%, Swap: {:.2}%",
        (meminfo.used as f64 / meminfo.total as f64) * 100.0,
        (meminfo.swap_used as f64 / meminfo.swap_total as f64) * 100.0
    )
}

pub fn ram_router() -> Router {
    Router::new().route("/ram", get(ram))
}
