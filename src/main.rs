use crate::config::PORT;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::fmt;

mod config;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Disable unix socket
//     #[arg(long, default_value = "false")]
//     no_unix: bool,
// }

async fn hello(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn main() {
    // let args = Args::parse();

    fmt::init();

    info!("Starting server...");

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));

    let listener = TcpListener::bind(addr).await.expect("failed to bind");

    info!("Server started");

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
