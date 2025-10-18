use crate::http::controllers::application_controller::{delete_application, update_application};
use crate::{
    core::types::{AppState, Config, Root},
    http::controllers::application_controller::get_applications,
    http::controllers::certificate_controller::get_certificates,
    http::controllers::config_controller::{delete_config, get_config, update_config},
    http::controllers::listener_controller::{delete_listener, get_listeners, update_listener},
    http::controllers::route_controller::get_routes,
    http::controllers::status_controller::get_status,
    http::types::{ErrorMessage, Status},
};
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Bytes, Incoming};
use hyper::header::HeaderValue;
use hyper::http::response::Builder;
use hyper::{Method, Request, Response, StatusCode};
use std::sync::Arc;

mod controllers;
pub mod server;
pub mod types;

pub fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn make_response() -> Builder {
    Response::builder()
        .header("Content-Type", HeaderValue::from_static("application/json"))
        .header("X-Powered-By", HeaderValue::from_static("Lodelix"))
        .header(
            "X-App-Version",
            HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
        )
}

async fn router(
    req: Request<Incoming>,
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    match (&method, path.as_str()) {
        (&Method::GET, "/") => get_root(state).await,
        (&Method::GET, "/config") => get_config(state).await,
        (&Method::PUT, "/config") => update_config(req, state).await,
        (&Method::DELETE, "/config") => delete_config(state).await,
        (&Method::GET, "/config/listeners") => get_listeners(state).await,
        (&Method::PUT, path) if path.starts_with("/config/listeners/") => {
            let name = path.trim_start_matches("/config/listeners/");
            update_listener(req, state, name).await
        }
        (&Method::DELETE, path) if path.starts_with("/config/listeners/") => {
            let name = path.trim_start_matches("/config/listeners/");
            delete_listener(state, name).await
        }
        (&Method::GET, "/config/routes") => get_routes(state).await,
        (&Method::GET, "/config/applications") => get_applications(state).await,
        (&Method::PUT, path) if path.starts_with("/config/applications/") => {
            let name = path.trim_start_matches("/config/applications/");
            update_application(state, name).await
        }
        (&Method::DELETE, path) if path.starts_with("/config/applications/") => {
            let name = path.trim_start_matches("/config/applications/");
            delete_application(state, name).await
        }
        (&Method::GET, "/certificates") => get_certificates().await,
        (&Method::GET, "/status") => get_status(state).await,
        _ => not_found().await,
    }
}

pub async fn get_root(
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    // TODO: get config from state
    // let _config = state.config.read().unwrap();

    let root = Root {
        config: Config::default(),
        status: Status {
            version: state.version.clone(),
            start_time: state.start_time,
        },
    };

    let response = make_response()
        .status(StatusCode::OK)
        .body(full(serde_json::to_vec(&root).unwrap()))
        .unwrap();

    Ok(response)
}

async fn not_found() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let response = ErrorMessage {
        message: "Resource not found".to_string(),
    };

    let response = make_response()
        .status(StatusCode::NOT_FOUND)
        .body(full(serde_json::to_vec(&response).unwrap()))
        .unwrap();

    Ok(response)
}
