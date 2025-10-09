use crate::grpc::proto::config_service_server::ConfigService;
use crate::grpc::proto::{ConfigRequest, ConfigResponse};
use crate::grpc::server::{ConfigHandler, StatusHandler};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl ConfigService for ConfigHandler {
    async fn get_config(
        &self,
        request: Request<ConfigRequest>,
    ) -> Result<Response<ConfigResponse>, Status> {
        let state = self.state.lock().await;

        let reply = ConfigResponse {
            test: "test".to_string(),
            listeners: std::collections::HashMap::new(),
            routes: "test".to_string(),
            applications: "test".to_string(),
        };

        Ok(Response::new(reply))
    }
}
