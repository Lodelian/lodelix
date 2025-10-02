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
use std::str::FromStr;
use tokio::net::TcpListener;

use clap::Parser;
use tracing::{error, info};

#[derive(Debug, Clone)]
enum ControlAddress {
    Tcp(SocketAddr),
    Unix(String),
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

pub async fn serve() {
    let args = Args::parse();

    info!("Starting server...");

    if let Some(ref control) = args.control {
        match control {
            ControlAddress::Tcp(addr) => {

                info!("Control API socket address: {}", addr);
            }
            ControlAddress::Unix(path) => {
                info!("Control API unix socket: {}", path);
            }
        }
    }

    #[cfg(windows)]
    run_named_pipe();

    #[cfg(unix)]
    run_unix_socket().await;

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
