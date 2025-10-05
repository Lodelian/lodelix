use crate::config::GRPC_PORT;
use crate::grpc::proto::status_service_server::StatusServiceServer;
use crate::grpc::status::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tracing::info;

#[derive(Clone)]
pub struct StatusHandler {
    pub(crate) state: Arc<Mutex<AppState>>,
}

pub async fn start_grpc_server(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let handler = StatusHandler {
        state: Arc::new(Mutex::new((*state).clone())),
    };

    info!("gRPC server started on 0.0.0.0:{}", GRPC_PORT);

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], GRPC_PORT));

    Server::builder()
        .add_service(StatusServiceServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}
