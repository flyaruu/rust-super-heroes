pub mod location {
    tonic::include_proto!("io.quarkus.sample.superheroes.location.v1");
}

use std::{sync::Arc, time::Duration};

use axum::{extract::State, routing::get, Router};
use location::{locations_client::LocationsClient, RandomLocationRequest};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use superhero_types::{heroes::SqlHero, villains::SqlVillain};
use tokio::{sync::{Mutex, OnceCell}, time::sleep};
use tonic::transport::Channel;

#[derive(Debug, Clone)]
struct FightsState {
    // LocahtionsClient is clone, so just do that?
    locations_client: Arc<Mutex<LocationsClient<Channel>>>,
    http_client: reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fighters {
    hero: SqlHero,
    villain: SqlVillain,
}

#[tokio::main]
async fn main() {
    let locations_client: LocationsClient<Channel> = loop {
        match LocationsClient::connect("http://grpc-locations:50051").await {
            Ok(client) => break client,
            Err(e) => {
                info!("Not up yet, waiting...: {:?}",e);
                sleep(Duration::from_millis(100)).await;
            },
        }
    };
    let client = reqwest::Client::builder().build().unwrap();
    let state = FightsState {
        locations_client: Arc::new(Mutex::new(locations_client)),
        http_client: client,
    };
    let app = Router::new()
        .route("/api/fight/randomlocation", get(random_location))
        .route("/api/fight/randomfighters", get(random_fighters))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listener created");
    axum::serve(listener, app).await.unwrap();
}

async fn random_location(State(fight_state): State<FightsState>)->String {
    let client = &mut *fight_state.locations_client.lock().await;
        //.get_or_init(|| LocationsClient::connect("http://grpc-locations:50051"));
    let response = client.get_random_location(RandomLocationRequest::default()).await.unwrap();
    let location = response.into_inner();
    location.name
}

async fn random_fighters(State(fight_state): State<FightsState>)->String {
    let fighters = Fighters {
        hero: random_hero(&fight_state.http_client).await,
        villain: random_villain(&fight_state.http_client).await,
    };
    serde_json::to_string_pretty(&fighters).unwrap()
}

async fn random_hero(client: &Client)->SqlHero {
    let body = client.get("http://rest-heroes:3000/api/heroes/random_hero").send().await.unwrap().text().await.unwrap();
    serde_json::from_str(&body).unwrap()
}

async fn random_villain(client: &Client)->SqlVillain {
    let body = client.get("http://rest-villains:3000/api/villains/random_villain").send().await.unwrap().text().await.unwrap();
    serde_json::from_str(&body).unwrap()
}