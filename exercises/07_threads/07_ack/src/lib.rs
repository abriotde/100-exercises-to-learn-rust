use std::sync::mpsc::{Receiver, Sender};
use crate::store::{TicketStore, TicketId};
use crate::data::Ticket;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert(data::TicketDraft, std::sync::mpsc::Sender<TicketId>),
    Get(TicketId, std::sync::mpsc::Sender<Result<Ticket, ()>>)
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert(ticket, sender)) => {
                println!("Insert ticket {:?}", ticket);
                let id = store.add_ticket(ticket);
                println!("Insert ticket => {:?}", id);
                sender.send(id).unwrap();
            }
            Ok(Command::Get(id, sender)) => {
                let res = store.get(id);
                let s = match res {
                    Some(ticket) => Ok(ticket.clone()),
                    _ => Err(())
                };
                sender.send(s).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
