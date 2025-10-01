#[cfg(windows)]
use {crate::config::PIPE_NAME, tokio::net::windows::named_pipe::ServerOptions};

#[cfg(unix)]
use {
    crate::config::UNIX_SOCKET,
    std::{fs, path::Path},
    tokio::net::UnixSocket,
};

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

    #[cfg(windows)]
    run_named_pipe();

    #[cfg(unix)]
    run_unix_socket().await;

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

#[cfg(windows)]
async fn run_named_pipe() {
    let pipe = ServerOptions::new().create(PIPE_NAME).unwrap();

    info!("Named pipe lodelix started");

    // Wait for a client to connect.
    pipe.connect().await.expect("TODO: panic message");
}

#[cfg(unix)]
async fn run_unix_socket() {
    if Path::exists(UNIX_SOCKET.as_ref()) {
        fs::remove_file(UNIX_SOCKET).unwrap();
    }

    let socket: UnixSocket = UnixSocket::new_stream().unwrap();

    socket.bind(UNIX_SOCKET).expect("TODO: panic message");

    info!("Unix socket lodelix started");
}
