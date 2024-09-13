use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}
