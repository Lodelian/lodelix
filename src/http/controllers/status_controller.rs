use std::sync::Arc;
use crate::http::types::Status;
use crate::http::{full, make_response};
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
use crate::core::types::AppState;

pub async fn get_status(
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let status: Status = Status {
        version: state.version.clone(),
        start_time: state.start_time,
    };

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&status).unwrap()))
        .unwrap();

    Ok(response)
}
