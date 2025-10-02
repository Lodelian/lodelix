use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub(crate) version: String,
    pub(crate) uptime: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub(crate) config: (),
    pub(crate) status: Status,
    pub(crate) _links: (),
}
