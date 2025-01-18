use std::fs;
use axum::{routing::get, Router, response::IntoResponse};

#[derive(Debug, Default)]
struct NetStat {
    rx_bytes: u64,
    rx_packets: u64,
    tx_bytes: u64,
    tx_packets: u64,
}

fn read_netdev() -> Vec<(String, NetStat)> {
    let content = fs::read_to_string("/proc/net/dev").expect("Failed to read /proc/net/dev");
    content.lines().skip(2).filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 17 {
            return None;
        }
        let interface = parts[0].trim_end_matches(':').to_string();
        Some((
            interface,
            NetStat {
                rx_bytes: parts[1].parse().unwrap_or(0),
                rx_packets: parts[2].parse().unwrap_or(0),
                tx_bytes: parts[9].parse().unwrap_or(0),
                tx_packets: parts[10].parse().unwrap_or(0),
            },
        ))
    }).collect()
}

async fn net() -> impl IntoResponse {
    let net_stats = read_netdev();
    let mut result = String::new();
    for (iface, stat) in net_stats {
        result.push_str(&format!(
            "{}: RX={} bytes, TX={} bytes, RX Packets={}, TX Packets={}\n",
            iface, stat.rx_bytes, stat.tx_bytes, stat.rx_packets, stat.tx_packets
        ));
    }
    result
}

pub fn net_router() -> Router {
    Router::new().route("/net", get(net))
}
