/*
 * Copyright (C) Pavel Zavadski
 */
use std::sync::Arc;
use crate::http::server::serve;
use tracing_subscriber::fmt;

mod config;
mod core;
mod grpc;
mod http;

use crate::grpc::status::{AppState, start_grpc};

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
        serve(http_state).await;
    });

    let grpc = tokio::spawn(async {
        start_grpc(grpc_state).await.unwrap();
    });

    tokio::try_join!(http, grpc)?;
    Ok(())
}
