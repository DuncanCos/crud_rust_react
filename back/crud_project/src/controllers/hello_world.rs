use sqlx::prelude::*;
use axum::{
    response::IntoResponse,
   
   
};


pub async fn hello_world() -> impl IntoResponse {
   
            "bonjour le  world".to_owned().into_response()
        
}
