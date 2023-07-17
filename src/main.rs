use std::net::SocketAddr;

use axum::extract::Path;
use axum::extract::Query;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello-world", get(handler_hello_query))
        .route("/hello-world/:name", get(handler_hello_path));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!(
        "->> {:<12} - handler_hello query param - {params:?}",
        "HANDLER"
    );

    let name = params.name.as_deref().unwrap_or("World!!");
    let resp = format!("Hello <strong>{name}!!</strong>");
    Html(resp)
}

async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    println!(
        "->> {:<12} - handler_hello path variable - {name}",
        "HANDLER"
    );

    let resp = format!("Hello path param <strong>{name}</strong>");
    Html(resp)
}
