pub mod location {
    tonic::include_proto!("io.quarkus.sample.superheroes.location.v1");
}

use std::{env, str::FromStr, sync::Arc, time::Duration};

use axum::{Json, extract::State};
use location::{Location, RandomLocationRequest, locations_client::LocationsClient};
use log::info;
use reqwest::Client;
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use superhero_types::{
    fights::{FightRequest, FightResult, Fighters, Winner},
    heroes::SqlHero,
    villains::SqlVillain,
};
use tokio::{sync::Mutex, time::sleep};
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct FightsState {
    pub locations_client: Arc<Mutex<LocationsClient<Channel>>>,
    pub http_client: reqwest::Client,
    pub heroes_base_url: String,
    pub villains_base_url: String,
    pub pool: Arc<Pool<Sqlite>>,
}

pub const DEFAULT_HEROES_BASE_URL: &str = "http://localhost:8080";
pub const DEFAULT_VILLAINS_BASE_URL: &str = "http://localhost:8081";
pub const DEFAULT_LOCATIONS_BASE_URL: &str = "http://localhost:50051";

const DEFAULT_DATABASE_URL: &str = "sqlite:///tmp/db/fights.db";

pub async fn initialize() -> FightsState {
    let database_url =
        env::var("FIGHTS_DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned());
    let options = SqliteConnectOptions::from_str(&database_url)
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(options)
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
    FightsState {
        locations_client: Arc::new(Mutex::new(locations_client)),
        http_client: client,
        heroes_base_url,
        villains_base_url,
        pool: Arc::new(pool),
    }
}

fn env_base_url(variable: &str, default: &str) -> String {
    env::var(variable).unwrap_or_else(|_| default.to_owned())
}

pub async fn post_fight(
    State(fight_state): State<FightsState>,
    Json(request): Json<FightRequest>,
) -> Json<FightResult> {
    let result: FightResult = execute_fight(&request, &fight_state);
    insert_fight_result(&fight_state.pool, &result).await;
    Json(result)
}

async fn initialize_fights(pool: &Pool<Sqlite>) {
    sqlx::raw_sql(
        r#"
        CREATE TABLE IF NOT EXISTS fights (
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

#[inline(never)]
fn execute_fight(request: &FightRequest, _fight_state: &FightsState) -> FightResult {
    let winner = if request.hero.level >= request.villain.level {
        Winner::Heroes
    } else {
        Winner::Villains
    };
    FightResult::new(winner, &request.hero, &request.villain, &request.location)
}

pub async fn random_location(State(fight_state): State<FightsState>) -> Json<Location> {
    let client = &mut *fight_state.locations_client.lock().await;
    let response = client
        .get_random_location(RandomLocationRequest::default())
        .await
        .unwrap();
    Json(response.into_inner())
}

pub async fn random_fighters(State(fight_state): State<FightsState>) -> Json<Fighters> {
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
    use superhero_types::fights::FightRequest;

    #[test]
    fn test_parse_fight_request() {
        let bytes = include_bytes!("../resources/fight_request.json");
        let _parsed: FightRequest = serde_json::from_slice(bytes).expect("parse failed");
    }
}
