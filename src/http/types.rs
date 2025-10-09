use crate::core::types::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub(crate) version: String,
    pub(crate) uptime: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub(crate) message: String,
}
