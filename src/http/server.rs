#[cfg(windows)]
use {crate::config::PIPE_NAME, tokio::net::windows::named_pipe::ServerOptions};

#[cfg(unix)]
use {
    crate::config::UNIX_SOCKET,
    std::{fs, path::Path},
    tokio::net::UnixListener,
};

use crate::config::PORT;
use crate::http::router;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::core::types::{AppState, ControlAddress};
use clap::Parser;
use tracing::{error, info};

pub async fn start_http_server(control: Option<ControlAddress>, state: Arc<AppState>) {
    if let Some(ref control) = control {
        match control {
            ControlAddress::Tcp(addr) => {
                handle_tcp_listener(Some(addr.clone()), state).await;
            }
            ControlAddress::Unix(_path) => {
                #[cfg(unix)]
                {
                    info!("Control API unix socket: {}", _path);
                    handle_unix_listener(Some(_path.clone()), state).await;
                }
            }
            #[cfg(windows)]
            ControlAddress::NamedPipe(_path) => {
                info!("Control API named pipe: {}", _path);
                handle_named_pipe().await;
            }
        }
    } else {
        handle_tcp_listener(None, state).await;
    }
}

async fn handle_tcp_listener(addr: Option<SocketAddr>, state: Arc<AppState>) {
    let addr = addr.unwrap_or(SocketAddr::from(([0, 0, 0, 0], PORT)));
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("HTTP server started at http://0.0.0.0:{}", addr.port());

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| router(req, Arc::clone(&state))))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[cfg(windows)]
async fn handle_named_pipe() {
    let pipe = ServerOptions::new().create(PIPE_NAME).unwrap();

    info!("Named pipe lodelix started");

    // Wait for a client to connect.
    pipe.connect().await.expect("TODO: panic message");
}

#[cfg(unix)]
async fn handle_unix_listener(path: Option<String>, state: Arc<AppState>) {
    let path: String = path.unwrap_or(UNIX_SOCKET.to_string());

    if Path::exists(path.as_ref()) {
        fs::remove_file(path.to_string()).unwrap();
    }

    let socket: UnixListener = UnixListener::bind(path).expect("TODO: panic message");

    // TODO: fix Error serving connection: hyper::Error(Shutdown, Os { code: 57, kind: NotConnected, message: "Socket is not connected" })

    loop {
        let (stream, _) = socket.accept().await.unwrap();
        let io = TokioIo::new(stream);
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| router(req, Arc::clone(&state))))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
