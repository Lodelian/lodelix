/*
 * Copyright (C) Pavel Zavadski
 */
use crate::http::server::serve;
use tracing_subscriber::fmt;

mod config;
mod grpc;
mod http;

use crate::grpc::status::{AppState, start_grpc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt::init();

    let state = AppState {
        version: "1.0.0".to_string(),
        start_time: std::time::Instant::now(),
    };

    let http = tokio::spawn(async {
        serve().await;
    });

    let grpc = tokio::spawn(async move {
        start_grpc(state).await.unwrap();
    });

    tokio::try_join!(http, grpc)?;
    Ok(())
}
