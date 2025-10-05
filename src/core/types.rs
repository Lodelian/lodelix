use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Listener {
    pub(crate) pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub test: String,
    
    pub listeners: Option<HashMap<String, Listener>>,

    pub routes: Option<String>,

    pub applications: Option<String>,
}
