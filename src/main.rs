/*
 * Copyright (C) Pavel Zavadski
 */
use crate::http::server::serve;
use tracing_subscriber::fmt;

mod config;
mod http;

#[tokio::main]
async fn main() {
    // let args = Args::parse();

    fmt::init();

    serve().await;
}
