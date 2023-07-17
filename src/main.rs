use std::net::SocketAddr;

use axum::routing::get_service;
use axum::Router;
use tower_http::services::ServeDir;

mod error;
pub use self::error::{Error, Result};
mod web;
pub use self::web::routes_hello;
pub use self::web::routes_login;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello::routes())
        .merge(routes_login::routes())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
