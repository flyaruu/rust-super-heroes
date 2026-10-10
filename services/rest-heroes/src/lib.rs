use std::{env, str::FromStr, sync::Arc};

use log::info;
use sqlx::{
    Pool, Sqlite, query_as, query_scalar,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use superhero_types::heroes::SqlHero;

const HEROES_SQL: &str = include_str!("../../../database/heroes-db/init/heroes.sql");

#[derive(Clone)]
pub struct HeroesState {
    pub pool: Arc<Pool<Sqlite>>,
}

const DEFAULT_DATABASE_URL: &str = "sqlite:///tmp/db/heroes.db";

pub async fn initialize() -> HeroesState {
    let database_url = env::var("HEROES_DATABASE_URL")
        .unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned());
    let options = SqliteConnectOptions::from_str(&database_url)
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();
    initialize_heroes(&pool).await;
    info!("SQLite heroes database initialized");

    HeroesState {
        pool: Arc::new(pool),
    }
}


async fn initialize_heroes(pool: &Pool<Sqlite>) {
    sqlx::raw_sql(
        r#"
        CREATE TABLE IF NOT EXISTS Hero (
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

    let hero_count: i64 = query_scalar("SELECT COUNT(*) FROM Hero")
        .fetch_one(pool)
        .await
        .unwrap();
    if hero_count == 0 {
        seed_with_nextval(pool, HEROES_SQL, "hero_seq").await;
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

pub async fn query_random_hero(heroes_state: &HeroesState) -> Option<SqlHero> {
    query_as("select * from Hero order by random() limit 1")
        .fetch_optional(&*heroes_state.pool)
        .await
        .unwrap()
}

