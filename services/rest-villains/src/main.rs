use std::{sync::Arc, time::Duration};

use axum::{extract::State, routing::get, Router};
use sqlx::{postgres::PgPoolOptions, query_as, Pool, Postgres};
use superhero_types::villains::SqlVillain;

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
        .route("/api/villains", get(all_villains))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listener created");
    axum::serve(listener, app).await.unwrap();
    

}

async fn all_villains(State(heroes_state): State<VillainState>)->String {
    let pool = &*heroes_state.pool;
    println!("Querying...");
    let heroes: Vec<SqlVillain> = query_as("select * from villain").fetch_all(pool).await.unwrap();
    serde_json::to_string(&heroes).unwrap()
    // heroes.len().to_string()
}