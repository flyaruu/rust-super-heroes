pub mod location {
    tonic::include_proto!("io.quarkus.sample.superheroes.location.v1");
}

use std::{sync::Arc, time::Duration};

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use location::{locations_client::LocationsClient, Location, RandomLocationRequest};
use log::info;
use mongodb::{options::ClientOptions, Collection};
use rand::RngCore;
use reqwest::Client;
use superhero_types::{
    fights::{FightRequest, FightResult, Fighters, Winner}, heroes::SqlHero, villains::SqlVillain
};
use tokio::{sync::Mutex, time::sleep};
use tonic::transport::Channel;

#[derive(Debug, Clone)]
struct FightsState {
    // LocahtionsClient is clone, so just do that?
    locations_client: Arc<Mutex<LocationsClient<Channel>>>,
    http_client: reqwest::Client,
    mongo_client: mongodb::Client,
    // rng: ThreadRng,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let mongodb_url = "mongodb://super:super@localhost/?retryWrites=true";

    let client_options = ClientOptions::parse(mongodb_url).await.unwrap();
    // let client = Client::with_options(client_options).unwrap();
    let mongo_client = mongodb::Client::with_options(client_options).unwrap();
    let locations_client: LocationsClient<Channel> = loop {
        match LocationsClient::connect("http://localhost:50051").await {
            Ok(client) => break client,
            Err(e) => {
                info!("Not up yet, waiting...: {:?}", e);
                sleep(Duration::from_millis(100)).await;
            }
        }
    };

    let client = reqwest::Client::builder().build().unwrap();
    let state = FightsState {
        locations_client: Arc::new(Mutex::new(locations_client)),
        http_client: client,
        mongo_client: mongo_client,
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

async fn post_fight(
    State(fight_state): State<FightsState>,
    Json(request): Json<FightRequest>,
) -> Json<FightResult> {
    let result: FightResult = execute_fight(&request, &fight_state).await;
    let a: Collection<FightResult> = fight_state.mongo_client.database("fights").collection("fight_collection");
    a.insert_one(&result).await.unwrap();
    Json(result)
}

async fn execute_fight(request: &FightRequest, _fight_state: &FightsState) -> FightResult {
    let mut rng = rand::rng();
    let winner = if rng.next_u32() % 2 == 0 {
        Winner::Heroes
    } else {
        Winner::Villains
    };
    FightResult::new(winner, &request.hero, &request.villain, &request.location)
}

async fn random_location(State(fight_state): State<FightsState>) -> Json<Location> {
    let client = &mut *fight_state.locations_client.lock().await;
    let response = client
        .get_random_location(RandomLocationRequest::default())
        .await
        .unwrap();
    Json(response.into_inner())
}

async fn random_fighters(State(fight_state): State<FightsState>) -> Json<Fighters> {
    let fighters = Fighters {
        hero: random_hero(&fight_state.http_client).await,
        villain: random_villain(&fight_state.http_client).await,
    };
    Json(fighters)
}

async fn random_hero(client: &Client) -> SqlHero {
    client
        .get("http://rest-heroes:3000/api/heroes/random_hero")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn random_villain(client: &Client) -> SqlVillain {
    client
        .get("http://rest-villains:3000/api/villains/random_villain")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::FightRequest;

    #[test]
    fn test_parse_fight_request() {
        let bytes = include_bytes!("../resources/fight_request.json");
        let _parsed: FightRequest = serde_json::from_slice(bytes).expect("parse failed");
    }
}
