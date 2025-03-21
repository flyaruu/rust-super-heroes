use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, routing::get, Router};
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
        .route("/api/heroes", get(all_heroes))
        .route("/api/heroes/random_hero", get(random_hero))
        .route("/api/heroes/{id}", get(hero))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listener created");
    axum::serve(listener, app).await.unwrap();
    

}

// Path(user_id): Path<Uuid>

async fn hero(Path(id): Path<i64>, State(heroes_state): State<HeroesState>)->(StatusCode,String) {
    println!("User: {}",id);
    let hero: Option<SqlHero> = query_as("select * from Hero where id=$1").bind(id)
        .fetch_optional(&*heroes_state.pool).await
        .unwrap();
    if let Some(hero) = hero {
        (StatusCode::OK,serde_json::to_string(&hero).unwrap())
    } else {
        (StatusCode::NOT_FOUND,"Not found".to_owned())
    }

}

async fn random_hero(State(heroes_state): State<HeroesState>)->(StatusCode,String) {
    let hero: Option<SqlHero> = query_as("select * from Hero order by random() limit 1")
        .fetch_optional(&*heroes_state.pool).await
        .unwrap();
    if let Some(hero) = hero {
        (StatusCode::OK,serde_json::to_string(&hero).unwrap())
    } else {
        (StatusCode::NOT_FOUND,"Not found".to_owned())
    }
}

async fn all_heroes(State(heroes_state): State<HeroesState>)->String {
    let heroes: Vec<SqlHero> = query_as("select * from Hero").fetch_all(&*heroes_state.pool).await.unwrap();
    serde_json::to_string(&heroes).unwrap()
}