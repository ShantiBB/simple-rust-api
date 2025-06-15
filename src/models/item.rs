use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ItemPayload {
    pub name: String,
    pub description: String,
}
