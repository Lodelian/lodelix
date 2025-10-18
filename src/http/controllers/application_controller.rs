use crate::core::types::{AppState, Application, Config};
use crate::http::types::SuccessMessage;
use crate::http::{full, make_response};
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};

pub async fn get_applications(
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let config: RwLockReadGuard<Config> = state.config.read().unwrap();

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&config.applications).unwrap()))
        .unwrap();

    Ok(response)
}

pub async fn update_application(
    state: Arc<AppState>,
    name: &str,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let application_data = Application {
        type_: String::from("php84"),
    };

    let mut config: RwLockWriteGuard<Config> = state.config.write().unwrap();

    if let Some(ref mut applications) = config.applications {
        applications.insert(name.to_string(), application_data);
    }

    let response = SuccessMessage {
        message: "Application updated".to_string(),
    };

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&response).unwrap()))
        .unwrap();

    Ok(response)
}

pub async fn delete_application(
    state: Arc<AppState>,
    name: &str,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut config: RwLockWriteGuard<Config> = state.config.write().unwrap();

    if let Some(ref mut applications) = config.applications {
        applications.remove(name);
    }

    let response = SuccessMessage {
        message: "Application deleted".to_string(),
    };

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(response))
        .unwrap();

    Ok(response)
}
