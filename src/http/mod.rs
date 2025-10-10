use crate::core::types::AppState;
use crate::http::endpoints::{
    delete_listener, get_applications, get_certificates, get_config, get_listeners, get_root,
    get_routes, get_status, not_found, update_config, update_listener,
};
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Bytes, Incoming};
use hyper::{Method, Request, Response};
use std::sync::Arc;

pub mod endpoints;
pub mod server;
pub mod types;

pub(crate) fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub(crate) fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

async fn router(
    req: Request<Incoming>,
    state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    match (&method, path.as_str()) {
        (&Method::GET, "/") => get_root().await,
        (&Method::GET, "/config") => get_config(state).await,
        (&Method::PUT, "/config") => update_config(req).await,
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
        (&Method::GET, "/certificates") => get_certificates().await,
        (&Method::GET, "/status") => get_status().await,
        _ => not_found().await,
    }
}
