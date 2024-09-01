use axum::{routing::get, Router};


#[path = "../controllers/hello_world.rs"] mod hello_world;

pub  fn routes() -> Router  {

    Router::new()
        .route("/", get(hello_world::hello_world))
}