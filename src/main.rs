/*
 * Copyright (C) Pavel Zavadski
 */
use crate::http::server::start_http_server;
use clap::Parser;
use std::sync::{Arc, RwLock};
use tracing::info;
use tracing_subscriber::fmt;

mod config;
mod core;
mod grpc;
mod http;

use crate::core::types::{AppState, Config};
use crate::grpc::server::start_grpc_server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable gRPC server
    #[arg(long, help = "Enable gRPC server")]
    grpc: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt::init();

    let state = Arc::new(AppState {
        version: env!("CARGO_PKG_VERSION").to_string(),
        start_time: std::time::Instant::now(),
        config: Arc::new(RwLock::new(Config::default())),
    });

    let http_state = state.clone();

    info!("Starting server...");

    let http = tokio::spawn(async move {
        start_http_server(http_state).await;
    });

    #[cfg(feature = "grpc")]
    {
        let args = Args::parse();
        let grpc_state = state.clone();

        if args.grpc {
            let grpc = tokio::spawn(async move {
                if let Err(e) = crate::grpc::server::start_grpc_server(grpc_state).await {
                    tracing::error!("gRPC server error: {}", e);
                }
            });
            tokio::try_join!(http, grpc)?;
        } else {
            http.await?;
        }
    }

    #[cfg(not(feature = "grpc"))]
    http.await?;

    Ok(())
}
