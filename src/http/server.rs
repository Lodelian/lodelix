use crate::config::{PIPE_NAME, PORT};
use crate::http::router;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info};

use tokio::net::windows::named_pipe::{ServerOptions};



pub async fn serve() {
    info!("Starting server...");

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = TcpListener::bind(addr).await.expect("failed to bind");

    info!("Server started on http://{}", addr);

    #[cfg(windows)]
    run_named_pipe().await;

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

async fn run_named_pipe() {

    let pipe = ServerOptions::new().create(PIPE_NAME).unwrap();

    info!("Named pipe lodelix started");

    // Wait for a client to connect.
    pipe.connect().await.expect("TODO: panic message");
}