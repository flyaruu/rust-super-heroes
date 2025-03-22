use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(sqlx::Type,Debug)]
pub enum SqlLocationType {
    CITY,PLANET, PLACE, ISLAND, COUNTRY, MOON
}


#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Default)]
pub struct SqlLocation {
    // pub id: i64,
    pub description: String,
    pub name: String,
    pub picture: String,
    // r#type: String, // TODO use enum
}
