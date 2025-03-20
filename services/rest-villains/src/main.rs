use std::{sync::Arc, time::Duration};

use axum::{extract::State, routing::get, serve, Router};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, query_as, Database, Pool, Postgres};

#[derive(FromRow, Debug, Serialize)]
struct SqlVillain {
    id: i64,
    level: i32,
    name: String,
    #[sqlx(rename = "othername")]
    #[serde(skip_serializing_if = "String::is_empty")]
    other_name: String,
    picture : String,
    powers: String,
}

#[derive(Clone)]
struct VillainState {
    pool: Arc<Pool<Postgres>>
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
        pool: Arc::new(pool)
        
    };
    let app = Router::new()
        // .route("/", get(|| async { "Hello, World!" }))
        .route("/api/villains", get(nr_of_heroes))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listener created");
    axum::serve(listener, app).await.unwrap();
    

}

async fn nr_of_heroes(State(heroes_state): State<VillainState>)->String {
    let pool = &*heroes_state.pool;
    println!("Querying...");
    let heroes: Vec<SqlVillain> = query_as("select * from villain").fetch_all(pool).await.unwrap();
    serde_json::to_string(&heroes).unwrap()
    // heroes.len().to_string()
}