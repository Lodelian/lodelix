use crate::http::{empty, full};
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
use serde_json::json;



pub async fn status() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let status = json!({
        "version": env!("CARGO_PKG_VERSION"),
        "status": "ok",
        "uptime": 0,
    });

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(full(status.to_string()))
        .unwrap();

    Ok(response)
}

pub async fn not_found() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(empty())
        .unwrap();

    Ok(response)
}
