use crate::model::{ModelController, Ticket, TicketIn};
use crate::{Result};
use axum::extract::Path;
use axum::routing::{delete, get, post};
use axum::Router;
use axum::{extract::State, Json};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", get(list_tickets))
        .route("/ticket/:id", delete(delete_ticket))
        .route("/ticket", post(create_ticket))
        .with_state(mc)
}

// region -- REST handlers
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_in): Json<TicketIn>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create ticket", "HANDLER");
    let ticket = mc.create_ticket(ticket_in).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list tickets", "HANDLER");
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete ticket", "HANDLER");
    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
// end region -- REST handlers
