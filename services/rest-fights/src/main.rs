pub mod location {
    tonic::include_proto!("io.quarkus.sample.superheroes.location.v1");
}

use std::{env, sync::Arc, time::Duration};

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use location::{Location, RandomLocationRequest, locations_client::LocationsClient};
use log::info;
use rand::RngCore;
use reqwest::Client;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use superhero_types::{
    fights::{FightRequest, FightResult, Fighters, Winner},
    heroes::SqlHero,
    villains::SqlVillain,
};
use tokio::{sync::Mutex, time::sleep};
use tonic::transport::Channel;

#[derive(Debug, Clone)]
struct FightsState {
    // LocahtionsClient is clone, so just do that?
    locations_client: Arc<Mutex<LocationsClient<Channel>>>,
    http_client: reqwest::Client,
    heroes_base_url: String,
    villains_base_url: String,
    pool: Arc<Pool<Sqlite>>,
    // rng: ThreadRng,
}

const DEFAULT_HEROES_BASE_URL: &str = "http://localhost:8080";
const DEFAULT_VILLAINS_BASE_URL: &str = "http://localhost:8081";
const DEFAULT_LOCATIONS_BASE_URL: &str = "http://localhost:50051";
const LISTEN_HOST: &str = "0.0.0.0";
const LISTEN_PORT: u16 = 8082;

#[tokio::main]
async fn main() {
    // do things
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    initialize_fights(&pool).await;
    info!("SQLite fights database initialized");

    let heroes_base_url = env_base_url("HEROES_BASE_URL", DEFAULT_HEROES_BASE_URL);
    let villains_base_url = env_base_url("VILLAINS_BASE_URL", DEFAULT_VILLAINS_BASE_URL);
    let locations_base_url = env_base_url("LOCATIONS_BASE_URL", DEFAULT_LOCATIONS_BASE_URL);

    let locations_client: LocationsClient<Channel> = loop {
        match LocationsClient::connect(locations_base_url.clone()).await {
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
        heroes_base_url,
        villains_base_url,
        pool: Arc::new(pool),
    };
    let app = Router::new()
        .route("/api/fights/randomlocation", get(random_location))
        .route("/api/fights/randomfighters", get(random_fighters))
        .route("/api/fights", post(post_fight))
        .with_state(state);

    // run our app with hyper, listening globally
    let listen_address = format!("{}:{}", LISTEN_HOST, LISTEN_PORT);
    let listener = tokio::net::TcpListener::bind(&listen_address)
        .await
        .unwrap();
    info!(
        "Fights service listening on host={} port={}",
        LISTEN_HOST, LISTEN_PORT
    );
    axum::serve(listener, app).await.unwrap();
}

fn env_base_url(variable: &str, default: &str) -> String {
    env::var(variable).unwrap_or_else(|_| default.to_owned())
}

async fn post_fight(
    State(fight_state): State<FightsState>,
    Json(request): Json<FightRequest>,
) -> Json<FightResult> {
    let result: FightResult = execute_fight(&request, &fight_state).await;
    insert_fight_result(&fight_state.pool, &result).await;
    Json(result)
}

async fn initialize_fights(pool: &Pool<Sqlite>) {
    sqlx::raw_sql(
        r#"
        CREATE TABLE fights (
          id TEXT NOT NULL PRIMARY KEY,
          fight_date TEXT NOT NULL,
          winner_name TEXT NOT NULL,
          winner_level INTEGER NOT NULL,
          winner_powers TEXT NOT NULL,
          winner_picture TEXT NOT NULL,
          winner_team TEXT NOT NULL,
          loser_name TEXT NOT NULL,
          loser_level INTEGER NOT NULL,
          loser_powers TEXT NOT NULL,
          loser_picture TEXT NOT NULL,
          loser_team TEXT NOT NULL,
          location_name TEXT NOT NULL,
          location_description TEXT NOT NULL,
          location_picture TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await
    .unwrap();
}

async fn insert_fight_result(pool: &Pool<Sqlite>, result: &FightResult) {
    sqlx::query(
        r#"
        INSERT INTO fights (
          id,
          fight_date,
          winner_name,
          winner_level,
          winner_powers,
          winner_picture,
          winner_team,
          loser_name,
          loser_level,
          loser_powers,
          loser_picture,
          loser_team,
          location_name,
          location_description,
          location_picture
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&result.id)
    .bind(&result.fight_date)
    .bind(&result.winner_name)
    .bind(result.winner_level)
    .bind(&result.winner_powers)
    .bind(&result.winner_picture)
    .bind(&result.winner_team)
    .bind(&result.loser_name)
    .bind(result.loser_level)
    .bind(&result.loser_powers)
    .bind(&result.loser_picture)
    .bind(&result.loser_team)
    .bind(&result.location.name)
    .bind(&result.location.description)
    .bind(&result.location.picture)
    .execute(pool)
    .await
    .unwrap();
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
        hero: random_hero(&fight_state.http_client, &fight_state.heroes_base_url).await,
        villain: random_villain(&fight_state.http_client, &fight_state.villains_base_url).await,
    };
    Json(fighters)
}

async fn random_hero(client: &Client, heroes_base_url: &str) -> SqlHero {
    client
        .get(format!(
            "{}/api/heroes/random_hero",
            heroes_base_url.trim_end_matches('/')
        ))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn random_villain(client: &Client, villains_base_url: &str) -> SqlVillain {
    client
        .get(format!(
            "{}/api/villains/random_villain",
            villains_base_url.trim_end_matches('/')
        ))
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
