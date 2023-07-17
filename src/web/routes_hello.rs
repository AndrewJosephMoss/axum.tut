use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn routes() -> Router {
    let routes_hello = Router::new()
        .route("/hello-world", get(handler_hello_query))
        .route("/hello-world/:name", get(handler_hello_path));
    routes_hello
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
