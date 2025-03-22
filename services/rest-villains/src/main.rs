use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, query_as};
use superhero_types::villains::SqlVillain;

#[derive(Clone)]
struct VillainState {
    pool: Arc<Pool<Postgres>>,
}

#[tokio::main]
async fn main() {
    println!("Main");

    let pool = PgPoolOptions::new()
        .idle_timeout(Duration::from_secs(1))
        .connect("postgres://superman:superman@villains-db:5432/villains_database")
        .await
        .unwrap();
    println!("Pool created");
    let state = VillainState {
        pool: Arc::new(pool),
    };
    let app = Router::new()
        // .route("/", get(|| async { "Hello, World!" }))
        .route("/api/villains", get(all_villains))
        .route("/api/villains/random_villain", get(random_villain))
        .route("/api/villains/{id}", get(villain))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listener created");
    axum::serve(listener, app).await.unwrap();
}

async fn villain(
    Path(id): Path<i64>,
    State(villain_state): State<VillainState>,
) -> (StatusCode, String) {
    println!("User: {}", id);
    let villain: Option<SqlVillain> = query_as("select * from villain where id=$1")
        .bind(id)
        .fetch_optional(&*villain_state.pool)
        .await
        .unwrap();
    if let Some(villain) = villain {
        (StatusCode::OK, serde_json::to_string(&villain).unwrap())
    } else {
        (StatusCode::NOT_FOUND, "Not found".to_owned())
    }
}

async fn random_villain(State(villain_state): State<VillainState>) -> (StatusCode, String) {
    let villain: Option<SqlVillain> = query_as("select * from villain order by random() limit 1")
        .fetch_optional(&*villain_state.pool)
        .await
        .unwrap();
    if let Some(villain) = villain {
        (StatusCode::OK, serde_json::to_string(&villain).unwrap())
    } else {
        (StatusCode::NOT_FOUND, "Not found".to_owned())
    }
}

async fn all_villains(State(villain_state): State<VillainState>) -> String {
    let pool = &*villain_state.pool;
    let villain: Vec<SqlVillain> = query_as("select * from villain")
        .fetch_all(pool)
        .await
        .unwrap();
    serde_json::to_string(&villain).unwrap()
}
