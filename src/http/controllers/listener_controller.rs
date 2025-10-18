use crate::core::types::AppState;
use crate::http::types::SuccessMessage;
use crate::http::{full, make_response};
use http_body_util::BodyExt;
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response, StatusCode};
use serde_json::json;
use std::sync::Arc;

pub async fn get_listeners(
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let config = state.config.read().unwrap();

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&config.listeners).unwrap()))
        .unwrap();

    Ok(response)
}

pub async fn update_listener(
    req: Request<Incoming>,
    state: Arc<AppState>,
    name: &str,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let body = req.collect().await?.to_bytes();
    let body_string = String::from_utf8_lossy(&body);

    // Parse the listener data from the request body
    let listener_data = serde_json::from_str(&body_string).unwrap();

    // Get write lock on config
    let mut config = state.config.write().unwrap();

    // Update the listener
    if let Some(ref mut listeners) = config.listeners {
        listeners.insert(name.to_string(), listener_data);
    }

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(
            serde_json::to_vec(&json!({"result": "Listener updated"})).unwrap(),
        ))
        .unwrap();

    Ok(response)
}

pub async fn delete_listener(
    state: Arc<AppState>,
    name: &str,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut config = state.config.write().unwrap();
    let listeners = config.listeners.as_mut().unwrap();
    listeners.remove(name);

    let response = SuccessMessage {
        message: "Listener deleted".to_string(),
    };

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(response))
        .unwrap();

    Ok(response)
}
