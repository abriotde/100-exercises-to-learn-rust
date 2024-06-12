use ticket_fields::{TicketDescription, TicketTitle};
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);
impl From<u64> for TicketId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl fmt::Display for TicketId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketJson {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: String,
}
impl TicketJson {
    pub fn new(ticket: Ticket) -> TicketJson {
        TicketJson {
            id: ticket.id.0  as i64,
            title: ticket.title.to_string(),
            description: ticket.description.to_string(),
            status: ticket.status.to_string(),  
        }
    }
    pub fn draft(self) -> TicketDraft {
        // TODO : Manage Errors
        TicketDraft {
            title: TicketTitle::try_from(self.title).unwrap(),
            description: TicketDescription::try_from(self.description).unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/*

http://127.0.0.1:8085/get?id=1

http://127.0.0.1:8085/add?draft={%22title%22:%22alberic%22,%20%22description%22:%22de%20la%20Crochais%22}

*/