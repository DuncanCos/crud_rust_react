use sqlx::mysql::MySqlPool;
use axum::response::IntoResponse;

pub async fn hello_world() -> impl IntoResponse{
   // let pool = MySqlPool::connect("mysql://dundun:dundun@localhost:3305/madb").await?;
    "bonjour le le world".to_owned()
}