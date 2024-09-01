mod hello_route;
mod users_route;


use axum::{ routing::get, Router};
//use hello_route::routes;


pub fn create_routes() -> Router {
    
    Router::new().merge(hello_route::routes())
        .merge(users_route::routes())

}