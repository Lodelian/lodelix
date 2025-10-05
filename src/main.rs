/*
 * Copyright (C) Pavel Zavadski
 */
use crate::grpc::server::start_grpc_server;
use crate::http::server::start_http_server;
use std::sync::Arc;
use tracing_subscriber::fmt;

mod config;
mod core;
mod grpc;
mod http;

use crate::grpc::status::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt::init();

    let state = Arc::new(AppState {
        version: env!("CARGO_PKG_VERSION").to_string(),
        start_time: std::time::Instant::now(),
    });

    let http_state = state.clone();
    let grpc_state = state.clone();

    let http = tokio::spawn(async {
        start_http_server(http_state).await;
    }).await;

    // TODO: make this configurable
    // let grpc = tokio::spawn(async {
    //     start_grpc_server(grpc_state).await.unwrap();
    // });

    // tokio::try_join!(http, grpc)?;
    Ok(())
}
