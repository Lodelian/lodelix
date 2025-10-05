pub mod server;
pub mod status;

pub mod proto {
    tonic::include_proto!("status");
}
