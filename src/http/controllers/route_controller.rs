use std::sync::Arc;
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
use crate::core::types::AppState;
use crate::http::{full, make_response};

pub async fn get_routes(
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let config = state.config.read().unwrap();

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&config.routes).unwrap()))
        .unwrap();

    Ok(response)
}
