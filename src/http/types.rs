use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Status {
    pub version: String,
    pub uptime: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub(crate) message: String,
}
