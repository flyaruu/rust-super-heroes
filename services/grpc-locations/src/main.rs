use std::result::Result;

use location::{locations_server::{Locations, LocationsServer}, DeleteAllLocationsResponse, HelloReply, LocationType, LocationsList};
use sqlx::{prelude::FromRow, query_as, MySql, MySqlPool, Pool};
use superhero_types::location::SqlLocation;
use tonic::{transport::Server, Request, Response, Status};

pub mod location {
    tonic::include_proto!("io.quarkus.sample.superheroes.location.v1");
}

struct MyLocations {
    pool: Pool<MySql>
}

#[tonic::async_trait]
impl Locations for MyLocations {

    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    async fn get_random_location(
        &self,
        _request: Request<location::RandomLocationRequest>,
    ) -> Result<tonic::Response<location::Location> ,tonic::Status> {
        let random: SqlLocation = query_as("select * from locations order by rand() limit 1").fetch_one(&self.pool).await
        .map_err(|e| Status::from_error(Box::new(e)))?;
        Ok(Response::new(random.into()))
    }
    
    async fn delete_all_locations(&self,_request:tonic::Request<location::DeleteAllLocationsRequest>) ->Result<tonic::Response<location::DeleteAllLocationsResponse>, tonic::Status> {
        let _: () = query_as("delete from locations").fetch_one(&self.pool).await.map_err(|e| Status::from_error(Box::new(e)))?;
        Ok(Response::new(DeleteAllLocationsResponse {}))
    }
    
    async fn get_location_by_name(&self,request:tonic::Request<location::GetLocationRequest> ,) -> Result<tonic::Response<location::Location>,tonic::Status> {
        let location: SqlLocation = query_as("select * from locations where name=? limit 1")
            .bind(request.into_inner().name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?
            .ok_or_else(|| tonic::Status::not_found("Can't find location"))?;
        Ok(Response::new(location.into()))
    }
    
    async fn replace_all_locations(&self, _request:tonic::Request<location::LocationsList> ,) -> Result<tonic::Response<location::ReplaceAllLocationsResponse>,tonic::Status> { //  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<tonic::Response<location::ReplaceAllLocationsResponse> ,tonic::Status, > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }
    
    async fn hello(&self,_request:tonic::Request<location::HelloRequest> ) -> Result<tonic::Response<location::HelloReply>,tonic::Status> {
        Ok(Response::new(HelloReply::default()))
    }
    
    async fn get_all_locations(&self, _request:tonic::Request<location::AllLocationsRequest>) -> Result<tonic::Response<LocationsList>,tonic::Status> {
        let all: Vec<SqlLocation> = query_as("select * from locations order by rand() limit 1").fetch_all(&self.pool).await
            .map_err(|e| tonic::Status::from_error(Box::new(e)))?;
        let items: Vec<location::Location> = all.into_iter().map(|item| item.into()).collect();
        Ok(Response::new(LocationsList { locations: items }))
    }

}

impl From<SqlLocation> for location::Location {
    fn from(value: SqlLocation) -> Self {
        // let location_type: i32 = match value.r#type.to_lowercase().as_str() {
        //     "city" => LocationType::City.into(),
        //     "planet" => LocationType::Planet.into(),
        //     "place" => LocationType::Place.into(),
        //     "island" => LocationType::Island.into(),
        //     "country" => LocationType::Country.into(),
        //     "moon" => LocationType::Moon.into(),
        //     _ => panic!("Unexpected type: {}",value.r#type)
        // };
        // TODO deal with the type
        Self { name: value.name, description: value.description, picture: value.picture, r#type: 0 }
    }
}

impl From<location::Location> for SqlLocation {
    fn from(value: location::Location) -> Self {
        // let sql_type = match value.r#type() {
        //     LocationType::Planet => "planet",
        //     LocationType::City => "city",
        //     LocationType::Place => "place",
        //     LocationType::Island => "island",
        //     LocationType::Country => "country",
        //     LocationType::Moon => "moon",
        // };
        SqlLocation { description: value.description, name: value.name, picture: value.picture }
    }
}

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect("mysql://locations:locations@locations-db/locations_database").await.unwrap();
    let core = MyLocations{ pool: pool };

    let addr = "[::]:50051".parse().unwrap();
    Server::builder()
        .add_service(LocationsServer::new(core))
        .serve(addr)
        .await
        .unwrap();
    // let a = tonic::server::builder();
        
    
    // let items: Vec<SqlLocation> = query_as("select * from locations").fetch_all(&pool).await.unwrap();
    // println!("Hello, world: {:?}",items);
}
