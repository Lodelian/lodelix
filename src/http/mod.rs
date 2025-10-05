use crate::http::endpoints::{
    get_certificates, get_config, get_root, get_status, not_found, update_config,
};
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Bytes, Incoming};
use hyper::{Method, Request, Response};

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
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => get_root().await,
        (&Method::GET, "/config") => get_config().await,
        (&Method::PUT, "/config") => update_config(req).await,
        (&Method::GET, "/certificates") => get_certificates().await,
        (&Method::GET, "/status") => get_status().await,
        _ => not_found().await,
    }
}
