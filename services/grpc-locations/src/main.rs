use grpc_locations::{initialize, run_server};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let core = initialize().await;
    run_server(core).await;
}
