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

use crate::core::types::AppState;
use clap::Parser;
use tracing::{error, info};

#[derive(Debug, Clone)]
enum ControlAddress {
    Tcp(SocketAddr),
    Unix(String),
    NamedPipe(String),
}

impl FromStr for ControlAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check for unix socket format
        if s.starts_with("unix:") {
            let path = s.strip_prefix("unix:").unwrap();
            if path.is_empty() {
                return Err("Unix socket path cannot be empty".to_string());
            }
            return Ok(ControlAddress::Unix(path.to_string()));
        }

        if s.starts_with("pipe:") {
            let path = s.strip_prefix("pipe:").unwrap();
            if path.is_empty() {
                return Err("Pipe path cannot be empty".to_string());
            }
            return Ok(ControlAddress::NamedPipe(path.to_string()));
        }

        // Try to parse as SocketAddr (IPv4 or IPv6)
        match s.parse::<SocketAddr>() {
            Ok(addr) => Ok(ControlAddress::Tcp(addr)),
            Err(_) => Err("Invalid control address format. Expected: IPv4 (127.0.0.1:8080), IPv6 ([::1]:8080), or Unix (unix:/path/to/socket)".to_string()),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Control API socket address in IPv4, IPv6, or UNIX domain format
    ///
    /// Examples:
    ///   - IPv4: 127.0.0.1:8080
    ///   - IPv6: ::1:8080
    ///   - Unix: unix:/path/to/control.unit.sock
    #[arg(long, value_name = "ADDRESS")]
    control: Option<ControlAddress>,
}

pub async fn start_http_server(state: Arc<AppState>) {
    let args = Args::parse();

    // TODO: fix control arg
    if let Some(ref control) = args.control {
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
            ControlAddress::NamedPipe(_path) => {
                #[cfg(windows)]
                {
                    info!("Control API named pipe: {}", _path);
                    handle_named_pipe().await;
                }
            }
        }
    } else {
        handle_tcp_listener(None, state).await;
    }
}

async fn handle_tcp_listener(addr: Option<SocketAddr>, state: Arc<AppState>) {
    let addr = addr.unwrap_or(SocketAddr::from(([0, 0, 0, 0], PORT)));
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server started at http://0.0.0.0:{}", addr.port());

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
