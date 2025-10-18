use http_body_util::BodyExt;
use std::collections::HashMap;
use crate::core::types::{AppState, Config};
use crate::http::{full, make_response};
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response, StatusCode};
use std::sync::Arc;
use hyper::body::{Bytes, Incoming};
use serde_json::json;

pub async fn get_config(
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let config = state.config.read().unwrap();

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&*config).unwrap()))
        .unwrap();

    Ok(response)
}

pub async fn update_config(
    req: Request<Incoming>,
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let body = req.collect().await?.to_bytes();
    let body_string = String::from_utf8_lossy(&body);
    let config = serde_json::from_str::<Config>(&body_string).unwrap();

    let mut state_config = state.config.write().unwrap();
    *state_config = config;

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&*state_config).unwrap()))
        .unwrap();

    Ok(response)
}

pub async fn delete_config(
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut config = state.config.write().unwrap();

    config.listeners = Some(HashMap::new());
    config.routes = Some(HashMap::new());
    config.applications = Some(HashMap::new());

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(
            serde_json::to_vec(&json!({"message": "Configuration deleted"})).unwrap(),
        ))
        .unwrap();

    Ok(response)
}
