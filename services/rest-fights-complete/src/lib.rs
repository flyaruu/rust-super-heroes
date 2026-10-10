use std::{env, str::FromStr, sync::Arc};

use axum::{Json, extract::State};
use log::info;
use rest_heroes::HeroesState;
use rest_villains::VillainState;
use sqlx::{
    Pool, Sqlite, query_as,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use superhero_types::{
    fights::{FightRequest, FightResult, Fighters, Winner},
    heroes::SqlHero,
    location::SqlLocation,
    villains::SqlVillain,
};

#[derive(Clone)]
pub struct FightsState {
    pub heroes_state: HeroesState,
    pub villains_state: VillainState,
    pub locations_state: Arc<grpc_locations::MyLocations>,
    pub pool: Arc<Pool<Sqlite>>,
}

const DEFAULT_DATABASE_URL: &str = "sqlite:///tmp/db/fights-complete.db";

pub async fn initialize() -> FightsState {
    let database_url = env::var("FIGHTS_COMPLETE_DATABASE_URL")
        .unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned());
    let options = SqliteConnectOptions::from_str(&database_url)
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();
    initialize_fights(&pool).await;
    info!("SQLite fights database initialized");

    let heroes_state = rest_heroes::initialize().await;
    let villains_state = rest_villains::initialize().await;
    let locations_state = Arc::new(grpc_locations::initialize().await);

    FightsState {
        heroes_state,
        villains_state,
        locations_state,
        pool: Arc::new(pool),
    }
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

pub async fn random_location(State(fight_state): State<FightsState>) -> Json<SqlLocation> {
    let location = grpc_locations::get_random_location(&fight_state.locations_state.pool)
        .await
        .unwrap();
    Json(location)
}

pub async fn random_fighters(State(fight_state): State<FightsState>) -> Json<Fighters> {
    let fighters = Fighters {
        hero: random_hero(&fight_state.heroes_state).await,
        villain: random_villain(&fight_state.villains_state).await,
    };
    Json(fighters)
}

async fn random_hero(heroes_state: &HeroesState) -> SqlHero {
    query_as("select * from Hero order by random() limit 1")
        .fetch_one(&*heroes_state.pool)
        .await
        .unwrap()
}

async fn random_villain(villains_state: &VillainState) -> SqlVillain {
    query_as("select * from villain order by random() limit 1")
        .fetch_one(&*villains_state.pool)
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
