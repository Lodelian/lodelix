use crate::http::{empty, full};
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Status {
    version: String,
    uptime: u64,
}

fn _get_status() -> Status {
    return Status {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: 0,
    };
}

pub async fn get_root() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(full(
            json!({
                "config": {},
                "status": {
                    "version": env!("CARGO_PKG_VERSION"),
                    "uptime": 0
                },
                "_links": {
                    "self": {
                        "href": "/"
                    },
                    "certificates": {
                        "href": "/certificates"
                    },
                    "status": {
                        "href": "/status"
                    }
                }
            })
            .to_string(),
        ))
        .unwrap();

    Ok(response)
}

pub async fn get_certificates() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(full(json!({}).to_string()))
        .unwrap();

    Ok(response)
}

pub async fn get_status() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let status: Status = _get_status();

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&status).unwrap()))
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
