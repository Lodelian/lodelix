use crate::http::types::Status;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct Listener {
    pub pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct Route {
    pub route_match: Match,
    pub route_action: Action,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub share: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone)]
pub struct AppState {
    pub version: String,
    pub start_time: SystemTime,
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
    pub routes: Option<HashMap<String, Route>>,
    pub applications: Option<HashMap<String, Application>>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            listeners: Some(HashMap::new()),
            routes: Some(HashMap::new()),
            applications: Some(HashMap::new()),
        }
    }
}
