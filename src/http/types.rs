use hyper::body::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Status {
    pub version: String,
    pub start_time: std::time::SystemTime,
}

#[derive(Serialize, Deserialize)]
pub struct SuccessMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub message: String,
}

impl From<SuccessMessage> for Bytes {
    fn from(msg: SuccessMessage) -> Self {
        let json = serde_json::to_vec(&msg).unwrap();
        Bytes::from(json)
    }
}

impl From<ErrorMessage> for Bytes {
    fn from(msg: ErrorMessage) -> Self {
        let json = serde_json::to_vec(&msg).unwrap();
        Bytes::from(json)
    }
}
