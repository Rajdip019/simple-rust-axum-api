use crate::{ctx::Ctx, model::{ModelController, Ticket, TicketForCreate}};
use axum::{extract::{FromRef, Path, State}, Json};
use crate::Result;
use axum::routing::{post, delete};
use axum::Router;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(app_state)
}

// region: --- REST Handlers

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!(">> HANDLER: create_ticket called!");
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!(">> HANDLER: list_tickets called!");
    let tickets = mc.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!(">> HANDLER: delete_ticket called!");
    let ticket = mc.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}

// endregion: --- REST Handlers