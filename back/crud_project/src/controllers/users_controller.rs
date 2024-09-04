use sqlx::prelude::*;
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    extract::Path,
    extract,
    Json,
};

#[path = "../database/dbtest.rs"] mod dbtest;
use serde::{Deserialize, Serialize}; // Importer la macro Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow,Serialize)]
struct Test {
    id: i32,
    name: String,
}

#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct Body {
    
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


pub async fn one_user(Path(id): extract::Path<i32>) -> impl IntoResponse {
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

pub async fn new_user(extract::Json(body): extract::Json<Body>)  -> impl IntoResponse {
    
    let pool = dbtest::dbtest().await;
    let result = sqlx::query_as::<_, Test>("INSERT INTO test (name) VALUES (?) ")
        .bind(body.name)
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

pub async fn modify_user( extract::Path(id): extract::Path<i32>, extract::Json(body): extract::Json<Body>)  -> impl IntoResponse {
    
    let pool = dbtest::dbtest().await;

   

    let result = sqlx::query_as::<_, Test>("UPDATE test SET name = ? WHERE id = ?")
        .bind(body.name)
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

pub async fn delete_user( extract::Path(id): extract::Path<i32>)  -> impl IntoResponse {
    
    let pool = dbtest::dbtest().await;

   

    let result = sqlx::query_as::<_, Test>("DELETE FROM test  WHERE id = ?")
        .bind(id)
        .fetch_all(&pool)
        .await;

        match result {
            Ok(rows) => {
                Json("deleted").into_response()
            }
            Err(_) => {
                // Si une erreur survient, retourner un code 500
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
}