pub mod location {
    tonic::include_proto!("io.quarkus.sample.superheroes.location.v1");
}

use std::{sync::Arc, time::Duration};

use axum::{extract::State, routing::{get, post}, Json, Router};
use location::{locations_client::LocationsClient, Location, RandomLocationRequest};
use log::{info, warn};
use reqwest::Client;
use serde::ser;
use superhero_types::{fights::{FightRequest, FightResult, Fighters, Winner}, heroes::SqlHero, location::SqlLocation, villains::SqlVillain};
use tokio::{sync::{Mutex, OnceCell}, time::sleep};
use tonic::transport::Channel;
use rand::{rngs::ThreadRng, Rng, RngCore};

#[derive(Debug, Clone)]
struct FightsState {
    // LocahtionsClient is clone, so just do that?
    locations_client: Arc<Mutex<LocationsClient<Channel>>>,
    http_client: reqwest::Client,
    // rng: ThreadRng,
}

#[tokio::main]
async fn main() {
    env_logger::init();

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
        .route("/api/fights/randomlocation", get(random_location))
        .route("/api/fights/randomfighters", get(random_fighters))
        .route("/api/fights", post(post_fight))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listener created");
    axum::serve(listener, app).await.unwrap();
}

async fn post_fight(State(fight_state): State<FightsState>, Json(request): Json<FightRequest>)->String {
    let result: FightResult = execute_fight(&request).await;
    let serialized = serde_json::to_string(&result).unwrap();
    serialized
}

async fn execute_fight(request: &FightRequest)->FightResult {
    let mut rng = rand::rng();
    let winner = 
    if rng.next_u32() % 2 == 0 {
        Winner::Heroes
    } else {
        Winner::Villains
    };
    FightResult::new(winner, &request.hero, &request.villain, &request.location)
}

async fn random_location(State(fight_state): State<FightsState>)->String {
    let client = &mut *fight_state.locations_client.lock().await;
        //.get_or_init(|| LocationsClient::connect("http://grpc-locations:50051"));
    let response = client.get_random_location(RandomLocationRequest::default()).await.unwrap();
    let location = response.into_inner();
    serde_json::to_string(&location).unwrap()
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


#[cfg(test)]
mod tests {
    use crate::FightRequest;

    #[test]
    fn test_parse_fight_request() {
        let bytes = include_bytes!("../resources/fight_request.json");
        let parsed: FightRequest = serde_json::from_slice(bytes).expect("parse failed");
    }
}