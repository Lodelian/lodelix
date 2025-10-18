use crate::http::types::Status;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
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

#[derive(Debug, Clone)]
pub enum ControlAddress {
    Tcp(SocketAddr),
    Unix(String),
    #[cfg(windows)]
    NamedPipe(String),
}

impl FromStr for ControlAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check for unix socket format
        if s.starts_with("unix:") {
            let path = s.strip_prefix("unix:").unwrap();
            if path.is_empty() {
                return Err("Unix socket path cannot be empty".to_string());
            }
            return Ok(ControlAddress::Unix(path.to_string()));
        }

        #[cfg(windows)]
        if s.starts_with("pipe:") {
            let path = s.strip_prefix("pipe:").unwrap();
            if path.is_empty() {
                return Err("Pipe path cannot be empty".to_string());
            }
            return Ok(ControlAddress::NamedPipe(path.to_string()));
        }

        // Try to parse as SocketAddr (IPv4 or IPv6)
        match s.parse::<SocketAddr>() {
            Ok(addr) => Ok(ControlAddress::Tcp(addr)),
            Err(_) => Err("Invalid control address format. Expected: IPv4 (127.0.0.1:8080), IPv6 ([::1]:8080), or Unix (unix:/path/to/socket)".to_string()),
        }
    }
}