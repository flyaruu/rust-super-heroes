pub mod location {
    tonic::include_proto!("io.quarkus.sample.superheroes.location.v1");
}

use std::sync::Arc;

use axum::{extract::State, routing::get, Router};
use location::{locations_client::LocationsClient, RandomLocationRequest};
use tokio::sync::Mutex;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
struct FightsState {
    // LocahtionsClient is clone, so just do that?
    client: Arc<Mutex<LocationsClient<Channel>>>
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client = LocationsClient::connect("http://localhost:50051").await.unwrap();
    let state = FightsState {
        client: Arc::new(Mutex::new(client))
    };
    // let response = client.get_random_location(RandomLocationRequest::default()).await.unwrap();
    // let location = response.into_inner();
    // println!("Location: {}",location.name);

    let app = Router::new()
    // .route("/", get(|| async { "Hello, World!" }))
    .route("/api/fight/randomlocation", get(random_location))
    .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    println!("Listener created");
    axum::serve(listener, app).await.unwrap();

    // location::locations_client::LocationsClient::new(inner)
}

async fn random_location(State(heroes_state): State<FightsState>)->String {
    let client = &mut *heroes_state.client.lock().await;
    let response = client.get_random_location(RandomLocationRequest::default()).await.unwrap();
    let location = response.into_inner();
    println!("Location: {}",location.name);
    // heroes.len().to_string()
    location.name
}