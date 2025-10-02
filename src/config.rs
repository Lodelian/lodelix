pub const PORT: u16 = 3000;

#[cfg(windows)]
pub const PIPE_NAME: &str = r"\\.\pipe\lodelix";

#[cfg(unix)]
pub const UNIX_SOCKET: &str = "/tmp/lodelix.sock";

pub const GRPC_PORT: u16 = 50051;
