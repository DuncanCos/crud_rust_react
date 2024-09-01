use axum::{routing::{get,post,put,delete}, Router};


#[path = "../controllers/users_controller.rs"] mod users_controller;

pub  fn routes() -> Router  {

    Router::new()
        .route("/users", get(users_controller::all_users))
        .route("/user/:id", get(users_controller::one_user))
}