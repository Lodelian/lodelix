use crate::config::GRPC_PORT;
use crate::grpc::proto::status_service_server::{StatusService, StatusServiceServer};
use crate::grpc::proto::{StatusRequest, StatusResponse};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Server};
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub version: String,
    pub start_time: std::time::Instant,
}

#[derive(Clone)]
pub struct StatusHandler {
    state: Arc<Mutex<AppState>>,
}

#[tonic::async_trait]
impl StatusService for StatusHandler {
    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let state = self.state.lock().await;

        let reply = StatusResponse {
            version: state.version.clone(),
            uptime: state.start_time.elapsed().as_secs() as i32,
        };

        Ok(Response::new(reply))
    }
}

pub async fn start_grpc(state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    let handler = StatusHandler {
        state: Arc::new(Mutex::new(state)),
    };

    info!("gRPC server started on 0.0.0.0:{}", GRPC_PORT);

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], GRPC_PORT));

    Server::builder()
        .add_service(StatusServiceServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}
