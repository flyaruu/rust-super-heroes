use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Serialize)]
pub struct SqlVillain {
    id: i64,
    level: i32,
    name: String,
    #[sqlx(rename = "othername")]
    #[serde(skip_serializing_if = "String::is_empty")]
    other_name: String,
    picture : String,
    powers: String,
}
