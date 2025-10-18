use crate::http::types::Status;
use crate::http::{full, make_response};
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};

fn _get_status() -> Status {
    Status {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: 0,
    }
}

pub async fn get_status() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let status: Status = _get_status();

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&status).unwrap()))
        .unwrap();

    Ok(response)
}
