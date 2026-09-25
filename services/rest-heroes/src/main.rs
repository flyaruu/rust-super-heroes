use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use log::info;
use sqlx::{Pool, Sqlite, query_as, sqlite::SqlitePoolOptions};
use superhero_types::heroes::SqlHero;

const HEROES_SQL: &str = include_str!("../../../database/heroes-db/init/heroes.sql");
const LISTEN_HOST: &str = "0.0.0.0";
const LISTEN_PORT: u16 = 8000;

#[derive(Clone)]
struct HeroesState {
    pool: Arc<Pool<Sqlite>>,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    initialize_heroes(&pool).await;
    info!("SQLite heroes database initialized");

    let state = HeroesState {
        pool: Arc::new(pool),
    };
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
    let hero: Option<SqlHero> = query_as("select * from Hero order by random() limit 1")
        .fetch_optional(&*heroes_state.pool)
        .await
        .unwrap();
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

async fn initialize_heroes(pool: &Pool<Sqlite>) {
    sqlx::raw_sql(
        r#"
        CREATE TABLE Hero (
          id INTEGER NOT NULL PRIMARY KEY,
          level INTEGER NOT NULL,
          name TEXT NOT NULL,
          othername TEXT,
          picture TEXT,
          powers TEXT
        );
        "#,
    )
    .execute(pool)
    .await
    .unwrap();

    seed_with_nextval(pool, HEROES_SQL, "hero_seq").await;
}

async fn seed_with_nextval(pool: &Pool<Sqlite>, seed_sql: &str, sequence_name: &str) {
    let marker = format!("nextval('{}')", sequence_name);
    let mut id = 1;

    for statement in seed_sql.split(';') {
        let Some(insert_start) = statement.find("INSERT INTO") else {
            continue;
        };
        let statement = statement[insert_start..].trim();
        let statement = statement.replace(&marker, &id.to_string());
        sqlx::query(&statement).execute(pool).await.unwrap();
        id += 50;
    }
}
