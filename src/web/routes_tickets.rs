use crate::ctx::Ctx;
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
    ctx: Ctx,
    Json(ticket_in): Json<TicketIn>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create ticket", "HANDLER");
    let ticket = mc.create_ticket(ctx, ticket_in).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list tickets", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}
// end region -- REST handlers
