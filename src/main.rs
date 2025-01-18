use axum::{
    routing::get,
    Router,
};
mod ram;
mod net;
mod disk;
mod cpu;

use ram::ram_router;
use net::net_router;
use disk::disk_router;
use cpu::cpu_router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(ram_router())
        .merge(net_router())
        .merge(disk_router())
        .merge(cpu_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}