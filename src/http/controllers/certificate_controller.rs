use crate::http::{full, make_response};
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
use serde_json::json;

pub async fn get_certificates() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = make_response()
        .status(StatusCode::OK)
        .body(full(json!({}).to_string()))
        .unwrap();

    Ok(response)
}
