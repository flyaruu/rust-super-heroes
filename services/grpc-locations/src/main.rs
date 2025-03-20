use location::{locations_server::{Locations, LocationsServer}, LocationType};
use sqlx::{prelude::FromRow, query_as, MySql, MySqlPool, Pool};
use tonic::{transport::Server, Request, Response};

pub mod location {
    tonic::include_proto!("io.quarkus.sample.superheroes.location.v1");
}

struct MyLocations {
    pool: Pool<MySql>
}

#[tonic::async_trait]
impl Locations for MyLocations {

    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
//    async fn get_random_location<'life0,'async_trait>(&self,request:tonic::Request<location::RandomLocationRequest> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = std::result::Result<tonic::Response<location::Location> ,tonic::Status> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {

//         todo!()
//     }

    async fn get_random_location(
        &self,
        request: Request<location::RandomLocationRequest>,
    ) -> std::result::Result<tonic::Response<location::Location> ,tonic::Status> {
        let random: SqlLocation = query_as("select * from locations order by rand() limit 1").fetch_one(&self.pool).await
            .map_err(|e| tonic::Status::not_found("Can't find location"))?;
        Ok(Response::new(random.into()))
        // self.pool.
        // println!("Got a request from {:?}", request.remote_addr());
        // let reply = SqlLocation {
        //     id: 1,
        //     name: "bla".to_owned(),
        //     picture: "bla".to_owned(),
        //     description: "desc".to_owned(),
        //     r#type: "CITY".to_owned(),

        // };
        // let location = location::Location {
        //     name: "bla".to_owned(),
        //     picture: "bla".to_owned(),
        //     description: "desc".to_owned(),
        //     r#type: location::LocationType::Country.into(),
        // };
        // Ok(tonic::Response::new(location))
    }

    // async fn say_hello(
    //     &self,
    //     request: Request<HelloRequest>,
    // ) -> Result<Response<HelloReply>, Status> {
    //     println!("Got a request: {:?}", request);
    //     let reply = hello_world::HelloReply {
    //         message: format!("Hello {}!", request.into_inner().name),
    //     };
    //     Ok(Response::new(reply))
    // }
    
    
    // #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn delete_all_locations<'life0,'async_trait>(&'life0 self,request:tonic::Request<location::DeleteAllLocationsRequest> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = std::result::Result<tonic::Response<location::DeleteAllLocationsResponse> ,tonic::Status, > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn get_location_by_name<'life0,'async_trait>(&'life0 self,request:tonic::Request<location::GetLocationRequest> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = std::result::Result<tonic::Response<location::Location> ,tonic::Status> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn replace_all_locations<'life0,'async_trait>(&'life0 self,request:tonic::Request<location::LocationsList> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = std::result::Result<tonic::Response<location::ReplaceAllLocationsResponse> ,tonic::Status, > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn hello<'life0,'async_trait>(&'life0 self,request:tonic::Request<location::HelloRequest> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = std::result::Result<tonic::Response<location::HelloReply> ,tonic::Status> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn get_all_locations<'life0,'async_trait>(&'life0 self,request:tonic::Request<location::AllLocationsRequest> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = std::result::Result<tonic::Response<location::LocationsList> ,tonic::Status> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

}

#[derive(sqlx::Type,Debug)]
enum SqlLocationType {
    CITY,PLANET, PLACE, ISLAND, COUNTRY, MOON
}


#[derive(FromRow, Debug)]
struct SqlLocation {
    id: i64,
    description: String,
    name: String,
    picture: String,
    r#type: String, // TODO use enum
}

impl From<SqlLocation> for location::Location {
    fn from(value: SqlLocation) -> Self {
        let location_type: i32 = match value.r#type.to_lowercase().as_str() {
            "city" => LocationType::City.into(),
            "planet" => LocationType::Planet.into(),
            "place" => LocationType::Place.into(),
            "island" => LocationType::Island.into(),
            "country" => LocationType::Country.into(),
            "moon" => LocationType::Moon.into(),
            _ => panic!("Unexpected type: {}",value.r#type)
        };
        Self { name: value.name, description: value.description, picture: value.picture, r#type: location_type }
    }
}


#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect("mysql://locations:locations@localhost/locations_database").await.unwrap();
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
