use std::{env, str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use log::info;
use sqlx::{
    Pool, Sqlite, query_as, query_scalar,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use superhero_types::villains::SqlVillain;

const VILLAINS_SQL: &str = include_str!("../../../database/villains-db/init/villains.sql");

#[derive(Clone)]
pub struct VillainState {
    pub pool: Arc<Pool<Sqlite>>,
}

const DEFAULT_DATABASE_URL: &str = "sqlite:///tmp/db/villains.db";

pub async fn initialize() -> VillainState {
    let database_url =
        env::var("VILLAINS_DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned());
    let options = SqliteConnectOptions::from_str(&database_url)
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();
    initialize_villains(&pool).await;
    info!("SQLite villains database initialized");

    VillainState {
        pool: Arc::new(pool),
    }
}

pub async fn villain(
    Path(id): Path<i64>,
    State(villain_state): State<VillainState>,
) -> (StatusCode, Json<Option<SqlVillain>>) {
    let villain: Option<SqlVillain> = query_as("select * from villain where id=?")
        .bind(id)
        .fetch_optional(&*villain_state.pool)
        .await
        .unwrap();
    if let Some(villain) = villain {
        (StatusCode::OK, Json(Some(villain)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

pub async fn random_villain(
    State(villain_state): State<VillainState>,
) -> (StatusCode, Json<Option<SqlVillain>>) {
    let villain = query_random_villain(villain_state).await;
    if let Some(villain) = villain {
        (StatusCode::OK, Json(Some(villain)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

pub async fn query_random_villain(villain_state: VillainState) -> Option<SqlVillain> {
    query_as("select * from villain order by random() limit 1")
        .fetch_optional(&*villain_state.pool)
        .await
        .unwrap()
}

pub async fn all_villains(State(villain_state): State<VillainState>) -> Json<Vec<SqlVillain>> {
    Json(
        query_as("select * from villain")
            .fetch_all(&*villain_state.pool)
            .await
            .unwrap(),
    )
}

async fn initialize_villains(pool: &Pool<Sqlite>) {
    sqlx::raw_sql(
        r#"
        CREATE TABLE IF NOT EXISTS Villain (
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

    let villain_count: i64 = query_scalar("SELECT COUNT(*) FROM Villain")
        .fetch_one(pool)
        .await
        .unwrap();
    if villain_count == 0 {
        seed_with_nextval(pool, VILLAINS_SQL, "villain_seq").await;
    }
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
