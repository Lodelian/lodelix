use crate::config::PORT;
use crate::http::router;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info};

pub async fn serve() {
    info!("Starting server...");

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = TcpListener::bind(addr).await.expect("failed to bind");

    info!("Server started on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(router))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
