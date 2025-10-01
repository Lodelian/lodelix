use crate::http::server::serve;
use tracing_subscriber::fmt;

mod config;
mod http;
// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Disable unix socket
//     #[arg(long, default_value = "false")]
//     no_unix: bool,
// }

#[tokio::main]
async fn main() {
    // let args = Args::parse();

    fmt::init();

    serve().await;
}
