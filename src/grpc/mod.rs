#[cfg(feature = "grpc")]
pub mod server;

#[cfg(feature = "grpc")]
pub mod status;

#[cfg(feature = "grpc")]
pub mod config;

#[cfg(feature = "grpc")]
pub mod proto {
    tonic::include_proto!("status");
    tonic::include_proto!("config");
}
