/*
 * Copyright (C) Pavel Zavadski
 */
use crate::config::GRPC_ENABLED;
use crate::http::server::start_http_server;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::fmt;

mod config;
mod core;
mod grpc;
mod http;

use crate::core::types::AppState;
use crate::grpc::server::start_grpc_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt::init();

    let state = Arc::new(AppState {
        version: env!("CARGO_PKG_VERSION").to_string(),
        start_time: std::time::Instant::now(),
    });

    let http_state = state.clone();
    let grpc_state = state.clone();

    info!("Starting server...");

    let http = tokio::spawn(async move {
        start_http_server(http_state).await;
    });

    if GRPC_ENABLED {
        let grpc = tokio::spawn(async move {
            if let Err(e) = start_grpc_server(grpc_state).await {
                tracing::error!("gRPC server error: {}", e);
            }
        });
        tokio::try_join!(http, grpc)?;
    } else {
        http.await?;
    }

    Ok(())
}
