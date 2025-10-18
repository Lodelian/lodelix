use crate::grpc::proto::status_service_server::StatusService;
use crate::grpc::proto::{StatusRequest, StatusResponse};
use crate::grpc::server::StatusHandler;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl StatusService for StatusHandler {
    async fn get_status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let state = self.state.lock().await;

        let reply = StatusResponse {
            version: state.version.clone(),
            start_time: state.start_time
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };

        Ok(Response::new(reply))
    }
}
