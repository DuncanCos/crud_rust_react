use axum::{Extension, Router};
use sqlx::mysql::MySqlPool;
use tower_http::cors::{CorsLayer, Any};

mod users_routes;

pub fn routing(pool: MySqlPool) -> Router {

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/users", users_routes::user_routing())
        .layer(Extension(pool))
        .layer(cors);
    

    app
}
