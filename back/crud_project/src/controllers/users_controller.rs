use sqlx::prelude::*;
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    extract::Path,
    Json,
};

#[path = "../database/dbtest.rs"] mod dbtest;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow,Serialize)]
struct Test {
    id: i32,
    name: String,
}

pub async fn all_users() -> impl IntoResponse {
    let pool = dbtest::dbtest().await; // Utilisez .await ici pour obtenir le Pool

    let result = sqlx::query_as::<_, Test>("SELECT * FROM test")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(rows) => {
            Json(rows).into_response()
        }
        Err(_) => {
            // Si une erreur survient, retourner un code 500
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}


pub async fn one_user(Path(id): Path<i32>) -> impl IntoResponse {
    // ...
    let pool = dbtest::dbtest().await;
    let result = sqlx::query_as::<_, Test>("SELECT * FROM test WHERE id = ?")
        .bind(id)
        .fetch_all(&pool)
        .await;

        match result {
            Ok(rows) => {
                Json(rows).into_response()
            }
            Err(_) => {
                // Si une erreur survient, retourner un code 500
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
}