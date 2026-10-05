use axum::{Router, routing::get};
use log::info;
use rest_villains::{VillainState, all_villains, random_villain, villain};

const LISTEN_HOST: &str = "0.0.0.0";
const LISTEN_PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let state: VillainState = rest_villains::initialize().await;

    let app = Router::new()
        .route("/api/villains", get(all_villains))
        .route("/api/villains/random_villain", get(random_villain))
        .route("/api/villains/{id}", get(villain))
        .with_state(state);

    let listen_address = format!("{}:{}", LISTEN_HOST, LISTEN_PORT);
    let listener = tokio::net::TcpListener::bind(&listen_address)
        .await
        .unwrap();
    info!(
        "Villains service listening on host={} port={}",
        LISTEN_HOST, LISTEN_PORT
    );
    axum::serve(listener, app).await.unwrap();
    info!("Exiting villains service");
}
