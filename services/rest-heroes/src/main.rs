use std::{sync::Arc, time::Duration};

use axum::{
    extract::{Path, State}, http::StatusCode, routing::get, Json, Router
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, query_as};
use superhero_types::heroes::SqlHero;

#[derive(Clone)]
struct HeroesState {
    pool: Arc<Pool<Postgres>>,
}

#[tokio::main]
async fn main() {
    println!("Main");

    let pool = PgPoolOptions::new()
        .max_connections(30)
        .connect("postgres://superman:superman@heroes-db:5432/heroes_database")
        .await
        .unwrap();
    println!("Pool created");
    let state = HeroesState {
        pool: Arc::new(pool),
    };
    let app = Router::new()
        .route("/api/heroes", get(all_heroes))
        .route("/api/heroes/random_hero", get(random_hero))
        .route("/api/heroes/{id}", get(hero))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listener created");
    axum::serve(listener, app).await.unwrap();
}

async fn hero(
    Path(id): Path<i64>,
    State(heroes_state): State<HeroesState>,
) -> (StatusCode, Json<Option<SqlHero>>) {
    println!("User: {}", id);
    let hero: Option<SqlHero> = query_as("select * from Hero where id=$1")
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

async fn random_hero(State(heroes_state): State<HeroesState>) -> (StatusCode, Json<Option<SqlHero>>) {
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
    Json(query_as("select * from Hero")
        .fetch_all(&*heroes_state.pool)
        .await
        .unwrap())
}