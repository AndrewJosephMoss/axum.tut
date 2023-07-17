use std::net::SocketAddr;

use axum::response::Response;
use axum::routing::get_service;
use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
pub use self::error::{Error, Result};
mod web;
pub use self::web::routes_hello;
pub use self::web::routes_login;
mod model;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello::routes())
        .merge(routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
