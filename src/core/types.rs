use crate::http::types::Status;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Listener {
    pub pass: String,
}

#[derive(Clone)]
pub struct AppState {
    pub version: String,
    pub start_time: std::time::Instant,
}

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub config: Config,
    pub status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub test: String,

    pub listeners: Option<HashMap<String, Listener>>,
    pub routes: Option<String>,
    pub applications: Option<String>,
}
