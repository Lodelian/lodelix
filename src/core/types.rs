use crate::http::types::Status;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize)]
pub struct Listener {
    pub pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct Route {
    pub route_match: Match,
    pub route_action: Action

}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub share: Option<String>
}

#[derive(Clone)]
pub struct AppState {
    pub version: String,
    pub start_time: std::time::Instant,
    pub config: Arc<RwLock<Config>>,
}

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub config: Config,
    pub status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub listeners: Option<HashMap<String, Listener>>,
    pub routes: Option<HashMap<String, String>>,
    pub applications: Option<HashMap<String, String>>,
}

impl Config {
    pub(crate) fn default() -> Config {
        Config {
            listeners: Some(HashMap::new()),
            routes: Some(HashMap::new()),
            applications: Some(HashMap::new()),
        }
    }
}
