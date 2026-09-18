use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use log::info;
use sqlx::{Pool, Sqlite, query_as, sqlite::SqlitePoolOptions};
use superhero_types::villains::SqlVillain;

const VILLAINS_SQL: &str = include_str!("../../../database/villains-db/init/villains.sql");

#[derive(Clone)]
struct VillainState {
    pool: Arc<Pool<Sqlite>>,
}

#[tokio::main]
async fn main() {
    info!("Main");
    env_logger::init();
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    initialize_villains(&pool).await;
    info!("SQLite villains database initialized");
    let state = VillainState {
        pool: Arc::new(pool),
    };
    let app = Router::new()
        // .route("/", get(|| async { "Hello, World!" }))
        .route("/api/villains", get(all_villains))
        .route("/api/villains/random_villain", get(random_villain))
        .route("/api/villains/{id}", get(villain))
        .with_state(state);

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    info!("Listener created");
    axum::serve(listener, app).await.unwrap();
    info!("Exiting villains service");
}

async fn villain(
    Path(id): Path<i64>,
    State(villain_state): State<VillainState>,
) -> (StatusCode, Json<Option<SqlVillain>>) {
    println!("User: {}", id);
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

async fn initialize_villains(pool: &Pool<Sqlite>) {
    sqlx::raw_sql(
        r#"
        CREATE TABLE Villain (
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

    seed_with_nextval(pool, VILLAINS_SQL, "villain_seq").await;
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

async fn random_villain(
    State(villain_state): State<VillainState>,
) -> (StatusCode, Json<Option<SqlVillain>>) {
    let villain: Option<SqlVillain> = query_as("select * from villain order by random() limit 1")
        .fetch_optional(&*villain_state.pool)
        .await
        .unwrap();
    if let Some(villain) = villain {
        (StatusCode::OK, Json(Some(villain)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

async fn all_villains(State(villain_state): State<VillainState>) -> Json<Vec<SqlVillain>> {
    let pool = &*villain_state.pool;
    Json(
        query_as("select * from villain")
            .fetch_all(pool)
            .await
            .unwrap(),
    )
}
