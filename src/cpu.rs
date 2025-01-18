use std::{fs, thread, time::Duration};
use axum::{routing::get, Router, response::IntoResponse};

#[derive(Debug, Default, Clone)]
struct CpuStat {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
}

impl CpuStat {
    fn total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait
            + self.irq + self.softirq + self.steal + self.guest + self.guest_nice
    }

    fn active(&self) -> u64 {
        self.user + self.nice + self.system + self.irq + self.softirq
            + self.steal + self.guest + self.guest_nice
    }
}

fn read_cpu_stats() -> Vec<CpuStat> {
    let content = fs::read_to_string("/proc/stat").expect("Failed to read /proc/stat");
    content.lines().filter(|line| line.starts_with("cpu")).map(|line| {
        let parts: Vec<u64> = line.split_whitespace().skip(1).filter_map(|x| x.parse().ok()).collect();
        CpuStat {
            user: parts.get(0).copied().unwrap_or(0),
            nice: parts.get(1).copied().unwrap_or(0),
            system: parts.get(2).copied().unwrap_or(0),
            idle: parts.get(3).copied().unwrap_or(0),
            iowait: parts.get(4).copied().unwrap_or(0),
            irq: parts.get(5).copied().unwrap_or(0),
            softirq: parts.get(6).copied().unwrap_or(0),
            steal: parts.get(7).copied().unwrap_or(0),
            guest: parts.get(8).copied().unwrap_or(0),
            guest_nice: parts.get(9).copied().unwrap_or(0),
        }
    }).collect()
}

fn calculate_cpu_usage(prev: &Vec<CpuStat>, current: &Vec<CpuStat>) -> Vec<f64> {
    prev.iter().zip(current.iter()).map(|(p, c)| {
        let total_diff = c.total().saturating_sub(p.total());
        let active_diff = c.active().saturating_sub(p.active());
        if total_diff == 0 {
            0.0
        } else {
            (active_diff as f64 / total_diff as f64) * 100.0
        }
    }).collect()
}

async fn cpu() -> impl IntoResponse {
    let prev_stats = read_cpu_stats();
    thread::sleep(Duration::from_secs(1));
    let current_stats = read_cpu_stats();
    let usages = calculate_cpu_usage(&prev_stats, &current_stats);
    let mut result = String::new();
    for (i, usage) in usages.iter().enumerate() {
        if i == 0 {
            result.push_str(&format!("Total CPU usage: {:.2}%\n", usage));
        } else {
            result.push_str(&format!("CPU{} usage: {:.2}%\n", i - 1, usage));
        }
    }
    result
}

pub fn cpu_router() -> Router {
    Router::new().route("/cpu", get(cpu))
}
