use crate::data::{Status, Ticket, TicketDraft};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);
/* use actix_web::{HttpServer, HttpResponse};
use actix_web::web::{Path};
*/
// use ticket_fields::TicketTitle;
// use ticket_fields::TicketDescription;
#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        let ticket = Arc::new(RwLock::new(ticket));
        self.tickets.insert(id, ticket);
        id
    }
    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }
}
/*
#[post("/add")]
pub async fn add_api() -> HttpResponse {
    let mut store = TicketStore::new();
    let draft = TicketDraft{
        title: TicketTitle::try_from("Toto").unwrap(),
        description: TicketDescription::try_from("il va bien").unwrap(),
    };
    let id = store.add_ticket(draft);
    let ticket = store.get(id).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ticket)
}
#[get("/get/{id}")]
pub async fn get_api(path: Path<(String,)>) -> HttpResponse {
    let mut store = TicketStore::new();
    let draft = TicketDraft{
        title: TicketTitle::try_from("Toto").unwrap(),
        description: TicketDescription::try_from("il va bien").unwrap(),
    };
    let id = store.add_ticket(draft);
    let ticket = store.get(id).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ticket)
}
*/