use crate::core::types::{Config, Listener};
use crate::http::types::{ErrorMessage, Root, Status};
use crate::http::{empty, full};
use http_body_util::BodyExt;
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::header::HeaderValue;
use hyper::http::response::Builder;
use hyper::{Request, Response, StatusCode};
use log::{debug, info};
use serde_json::json;

fn _get_status() -> Status {
    Status {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: 0,
    }
}

fn _get_config() -> Config {
    Config {
        test: "".to_string(),
        listeners: None,
        routes: None,
        applications: None,
    }
}

fn _get_root() -> Root {
    Root {
        config: _get_config(),
        status: _get_status(),
        _links: (),
    }
}

fn _make_response() -> Builder {
    Response::builder()
        .header("Content-Type", HeaderValue::from_static("application/json"))
        .header("X-Powered-By", HeaderValue::from_static("Lodelix"))
        .header(
            "X-App-Version",
            HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
        )
}

pub async fn get_root() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = _make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&_get_root()).unwrap()))
        .unwrap();

    Ok(response)
}

pub async fn get_config() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = _make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&_get_config()).unwrap()))
        .unwrap();

    Ok(response)
}

pub async fn update_config(
    req: Request<Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let body = req.collect().await?.to_bytes();
    let body_string = String::from_utf8_lossy(&body);
    let config = serde_json::from_str::<Config>(&body_string).unwrap();

    // TODO: finish method
    info!("{}", serde_json::to_string_pretty(&config).unwrap());

    let response = _make_response()
        .status(StatusCode::OK)
        .body(empty())
        .unwrap();

    Ok(response)
}

pub async fn get_certificates() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = _make_response()
        .status(StatusCode::OK)
        .body(full(json!({}).to_string()))
        .unwrap();

    Ok(response)
}

pub async fn get_status() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let status: Status = _get_status();

    let response = _make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&status).unwrap()))
        .unwrap();

    Ok(response)
}

pub async fn not_found() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = ErrorMessage {
        message: "Endpoint not found".to_string(),
    };

    let response = _make_response()
        .status(StatusCode::NOT_FOUND)
        .body(full(serde_json::to_vec(&response).unwrap()))
        .unwrap();

    Ok(response)
}
