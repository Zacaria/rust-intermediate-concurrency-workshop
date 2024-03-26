use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BlogPost {
    id: i32,
    pub date: String,
    pub title: String,
    pub body: String,
    pub author: String,
}
