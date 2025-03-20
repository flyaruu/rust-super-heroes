use std::sync::Arc;

use axum::{extract::State, routing::get, Router};
use sqlx::{postgres::PgPoolOptions, query_as, Pool, Postgres};
use superhero_types::heroes::SqlHero;

#[derive(Clone)]
struct HeroesState {
    pool: Arc<Pool<Postgres>>
}

#[tokio::main]
async fn main() {
    println!("Main");

    let pool = PgPoolOptions::new()
        .connect("postgres://superman:superman@heroes-db:5432/heroes_database")
        .await
        .unwrap();
    println!("Pool created");
    let state = HeroesState {
        pool: Arc::new(pool)
        
    };
    let app = Router::new()
        // .route("/", get(|| async { "Hello, World!" }))
        .route("/api/heroes", get(nr_of_heroes))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listener created");
    axum::serve(listener, app).await.unwrap();
    

}

async fn nr_of_heroes(State(heroes_state): State<HeroesState>)->String {
    let pool = &*heroes_state.pool;
    let heroes: Vec<SqlHero> = query_as("select * from Hero").fetch_all(pool).await.unwrap();
    serde_json::to_string(&heroes).unwrap()
    // heroes.len().to_string()
}