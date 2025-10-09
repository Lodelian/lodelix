pub mod server;
pub mod status;
pub mod config;

pub mod proto {
    tonic::include_proto!("status");
    tonic::include_proto!("config");
}
