use std::result::Result;

use location::{
    DeleteAllLocationsResponse, HelloReply, LocationsList,
    locations_server::{Locations, LocationsServer},
};
use log::info;
use sqlx::{Pool, Sqlite, query, query_as, sqlite::SqlitePoolOptions};
use superhero_types::location::SqlLocation;
use tonic::{Request, Response, Status, transport::Server};

const LOCATIONS_SQL: &str =
    include_str!("../../../database/locations-db/init/initialize-tables.sql");

pub mod location {
    tonic::include_proto!("io.quarkus.sample.superheroes.location.v1");
}

struct MyLocations {
    pool: Pool<Sqlite>,
}

#[tonic::async_trait]
impl Locations for MyLocations {
    #[allow(
        mismatched_lifetime_syntaxes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    async fn get_random_location(
        &self,
        _request: Request<location::RandomLocationRequest>,
    ) -> Result<tonic::Response<location::Location>, tonic::Status> {
        let random: SqlLocation = query_as("select * from locations order by random() limit 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?;
        Ok(Response::new(random.into()))
    }

    async fn delete_all_locations(
        &self,
        _request: tonic::Request<location::DeleteAllLocationsRequest>,
    ) -> Result<tonic::Response<location::DeleteAllLocationsResponse>, tonic::Status> {
        query("delete from locations")
            .execute(&self.pool)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?;
        Ok(Response::new(DeleteAllLocationsResponse {}))
    }

    async fn get_location_by_name(
        &self,
        request: tonic::Request<location::GetLocationRequest>,
    ) -> Result<tonic::Response<location::Location>, tonic::Status> {
        let location: SqlLocation = query_as("select * from locations where name=? limit 1")
            .bind(request.into_inner().name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?
            .ok_or_else(|| tonic::Status::not_found("Can't find location"))?;
        Ok(Response::new(location.into()))
    }

    async fn replace_all_locations(
        &self,
        _request: tonic::Request<location::LocationsList>,
    ) -> Result<tonic::Response<location::ReplaceAllLocationsResponse>, tonic::Status> {
        //  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<tonic::Response<location::ReplaceAllLocationsResponse> ,tonic::Status, > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    async fn hello(
        &self,
        _request: tonic::Request<location::HelloRequest>,
    ) -> Result<tonic::Response<location::HelloReply>, tonic::Status> {
        Ok(Response::new(HelloReply::default()))
    }

    async fn get_all_locations(
        &self,
        _request: tonic::Request<location::AllLocationsRequest>,
    ) -> Result<tonic::Response<LocationsList>, tonic::Status> {
        let all: Vec<SqlLocation> = query_as("select * from locations")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| tonic::Status::from_error(Box::new(e)))?;
        let items: Vec<location::Location> = all.into_iter().map(|item| item.into()).collect();
        Ok(Response::new(LocationsList { locations: items }))
    }
}

impl From<SqlLocation> for location::Location {
    fn from(value: SqlLocation) -> Self {
        Self {
            name: value.name,
            description: value.description,
            picture: value.picture,
            r#type: 0,
        }
    }
}

impl From<location::Location> for SqlLocation {
    fn from(value: location::Location) -> Self {
        SqlLocation {
            description: value.description,
            name: value.name,
            picture: value.picture,
        }
    }
}

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    initialize_locations(&pool).await;

    let core = MyLocations { pool };
    info!("SQLite locations database initialized, starting gRPC locations service...");
    let addr = "[::]:50051".parse().unwrap();
    Server::builder()
        .add_service(LocationsServer::new(core))
        .serve(addr)
        .await
        .unwrap();
}

async fn initialize_locations(pool: &Pool<Sqlite>) {
    sqlx::raw_sql(
        r#"
        CREATE TABLE locations (
          id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
          description TEXT,
          name TEXT NOT NULL UNIQUE,
          picture TEXT,
          type TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await
    .unwrap();

    for statement in LOCATIONS_SQL.split(';') {
        let Some(insert_start) = statement.find("INSERT INTO") else {
            continue;
        };
        let statement = statement[insert_start..].trim();
        sqlx::query(statement).execute(pool).await.unwrap();
    }
}
