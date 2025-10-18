use std::collections::HashMap;
use crate::grpc::proto::config_service_server::ConfigService;
use crate::grpc::proto::{ConfigRequest, ConfigResponse};
use crate::grpc::server::ConfigHandler;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl ConfigService for ConfigHandler {
    async fn get_config(
        &self,
        _request: Request<ConfigRequest>,
    ) -> Result<Response<ConfigResponse>, Status> {
        let state = self.state.config.read().unwrap();

        let reply = ConfigResponse {
            listeners: HashMap::new(),
            routes: "test".to_string(),
            applications: HashMap::new(),
        };

        Ok(Response::new(reply))
    }
}
