use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use log::info;
use rest_heroes::{HeroesState, query_random_hero};
use sqlx::{Pool, Sqlite, query_as, sqlite::SqlitePoolOptions};
use superhero_types::heroes::SqlHero;

const LISTEN_HOST: &str = "0.0.0.0";
const LISTEN_PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    let state = rest_heroes::initialize().await;
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let app = Router::new()
        .route("/api/heroes", get(all_heroes))
        .route("/api/heroes/random_hero", get(random_hero))
        .route("/api/heroes/{id}", get(hero))
        .with_state(state);

    let listen_address = format!("{}:{}", LISTEN_HOST, LISTEN_PORT);
    let listener = tokio::net::TcpListener::bind(&listen_address)
        .await
        .unwrap();
    info!(
        "Heroes service listening on host={} port={}",
        LISTEN_HOST, LISTEN_PORT
    );
    axum::serve(listener, app).await.unwrap();
    info!("Exiting heroes service");
}

async fn hero(
    Path(id): Path<i64>,
    State(heroes_state): State<HeroesState>,
) -> (StatusCode, Json<Option<SqlHero>>) {
    let hero: Option<SqlHero> = query_as("select * from Hero where id=?")
        .bind(id)
        .fetch_optional(&*heroes_state.pool)
        .await
        .unwrap();
    if let Some(hero) = hero {
        (StatusCode::OK, Json(Some(hero)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

async fn random_hero(
    State(heroes_state): State<HeroesState>,
) -> (StatusCode, Json<Option<SqlHero>>) {
    let hero = query_random_hero(heroes_state).await;
    if let Some(hero) = hero {
        (StatusCode::OK, Json(Some(hero)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

async fn all_heroes(State(heroes_state): State<HeroesState>) -> Json<Vec<SqlHero>> {
    Json(
        query_as("select * from Hero")
            .fetch_all(&*heroes_state.pool)
            .await
            .unwrap(),
    )
}
