use std::net::SocketAddr;

use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get_service;
use axum::{middleware, Json, Router};
use ctx::Ctx;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;
pub use self::error::{Error, Result};
pub use self::web::routes_hello;
pub use self::web::routes_login;
use crate::log::log_request;
use crate::model::ModelController;
use crate::web::mw_auth;
use crate::web::routes_tickets;

#[tokio::main]
async fn main() -> Result<()> {
    let model_controller = ModelController::new().await?;

    let routes_api = routes_tickets::routes(model_controller.clone())
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello::routes())
        .merge(routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            model_controller.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error =
        service_error.map(|service_error| service_error.client_status_and_error());

    // -- If client error, build the new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!("    ->> client_error_body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error);

    println!("    ->> server log line - {uuid} - Error: {service_error:?}");

    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
