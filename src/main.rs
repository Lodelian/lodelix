/*
 * Copyright (C) Pavel Zavadski
 */
use crate::http::server::start_http_server;
use clap::Parser;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use tracing::info;
use tracing_subscriber::fmt;

mod config;
mod core;
mod grpc;
mod http;

use crate::core::types::{AppState, Config, ControlAddress};

#[cfg(feature = "grpc")]
use crate::grpc::server::start_grpc_server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable gRPC server
    #[cfg(feature = "grpc")]
    #[arg(long, help = "Enable gRPC server")]
    grpc: bool,

    /// Control API socket address in IPv4, IPv6, or UNIX domain format
    ///
    /// Examples:
    ///   - IPv4: 127.0.0.1:8080
    ///   - IPv6: ::1:8080
    ///   - Unix: unix:/path/to/control.unit.sock
    #[arg(long, value_name = "ADDRESS")]
    control: Option<ControlAddress>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt::init();

    let _args = Args::parse();

    let state = Arc::new(AppState {
        version: env!("CARGO_PKG_VERSION").to_string(),
        start_time: SystemTime::now(),
        config: Arc::new(RwLock::new(Config::default())),
    });

    let http_state = state.clone();

    info!("Starting server...");

    let http = tokio::spawn(async move {
        start_http_server(_args.control, http_state).await;
    });

    #[cfg(feature = "grpc")]
    {
        let grpc_state = state.clone();

        if _args.grpc {
            let grpc = tokio::spawn(async move {
                if let Err(e) = start_grpc_server(grpc_state).await {
                    tracing::error!("gRPC server error: {}", e);
                }
            });
            tokio::try_join!(http, grpc)?;
        } else {
            http.await?;
        }
    }

    #[cfg(not(feature = "grpc"))]
    {
        info!("gRPC server feature is not enabled in this build.");

        http.await?;
    }

    Ok(())
}
