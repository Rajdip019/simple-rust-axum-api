//! Simplistic model layer
// with mock-store layer

use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    cid: u64, // creator_id
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
    pub description: String,
}
// endregion: Ticket types

// region: Mock Controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self,ctx: Ctx, ticket: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket.title,
            description: ticket.description,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let tickets = self.tickets_store.lock().unwrap();
        Ok(tickets.iter().filter_map(|x| x.clone()).collect())
    }

    pub async fn delete_ticket(&self,_ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        if id as usize >= store.len() {
            return Err(Error::TicketDeleteFailIdNotFound { id });
        }
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

// endregion: Mock Controller