use crate::data::{Status, Ticket, TicketDraft, TicketId};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Mutex};

/* use actix_web::{HttpServer, HttpResponse};
use actix_web::web::{Path};
*/
// use ticket_fields::TicketTitle;
// use ticket_fields::TicketDescription;
#[derive(Clone)]
pub struct TicketStore {
    tickets: Arc<RwLock<BTreeMap<TicketId, Arc<RwLock<Ticket>>>>>,
    counter: Arc<Mutex<u64>>,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Arc::new(RwLock::new(BTreeMap::new())),
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn add_ticket(&self, ticket: TicketDraft) -> TicketId {
        let mut lock = self.counter.try_lock();
        if let Ok(ref mut max_id) = lock {
            **max_id += 1;
            let id = TicketId::from(**max_id);
            let ticket = Ticket {
                id,
                title: ticket.title,
                description: ticket.description,
                status: Status::ToDo,
            };
            let ticket = Arc::new(RwLock::new(ticket));
            self.tickets.write().unwrap()
                .insert(id, ticket);
            id
        } else {
            TicketId::from(0)
        }
    }
    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.read().unwrap()
            .get(&id).cloned()
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