use axum::{
    Router,
    routing::{get, post},
};
use log::info;
use rest_fights_complete::{post_fight, random_fighters, random_location};

const LISTEN_HOST: &str = "0.0.0.0";
const LISTEN_PORT: u16 = 8082;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let state = rest_fights_complete::initialize().await;
    let app = Router::new()
        .route("/api/fights/randomlocation", get(random_location))
        .route("/api/fights/randomfighters", get(random_fighters))
        .route("/api/fights", post(post_fight))
        .with_state(state);

    let listen_address = format!("{}:{}", LISTEN_HOST, LISTEN_PORT);
    let listener = tokio::net::TcpListener::bind(&listen_address)
        .await
        .unwrap();
    info!(
        "Fights-complete service listening on host={} port={}",
        LISTEN_HOST, LISTEN_PORT
    );
    axum::serve(listener, app).await.unwrap();
}
