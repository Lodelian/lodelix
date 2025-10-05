use crate::grpc::proto::status_service_server::StatusService;
use crate::grpc::proto::{StatusRequest, StatusResponse};
use crate::grpc::server::StatusHandler;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct AppState {
    pub version: String,
    pub start_time: std::time::Instant,
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


