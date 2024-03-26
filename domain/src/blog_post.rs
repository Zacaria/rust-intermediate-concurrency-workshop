use serde::{Deserialize, Serialize};
// derive FromRow in the server crate
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: i32,
    pub date: String,
    pub title: String,
    pub body: String,
    pub author: String,
}
