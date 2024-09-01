mod hello_route;


use axum::{ routing::get, Router};
//use hello_route::routes;


pub fn create_routes() -> Router {
    Router::new().merge(hello_route::routes())

}