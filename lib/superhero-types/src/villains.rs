use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct SqlVillain {
    // id: i64,
    pub level: i32,
    pub name: String,
    #[sqlx(rename = "othername")]
    // #[serde(skip_serializing_if = "String::is_empty")]
    pub other_name: Option<String>,
    pub picture: String,
    pub powers: String,
}
