// use ticket_fields::test_helpers::{ticket_description, ticket_title};
use outro_08;
use outro_08::store::TicketStore;
use outro_08::data::{Status, Ticket, TicketDraft};
use outro_08::server;
use ticket_fields::TicketTitle;
use ticket_fields::TicketDescription;

#[test]
fn server_test() {
    // server();
    let move_forward = true;
    server::run_server();
    assert!(move_forward);
}

#[test]
fn store() {
    let mut store = TicketStore::new();
    let draft = TicketDraft{
        title: TicketTitle::try_from("Toto").unwrap(),
        description: TicketDescription::try_from("il va bien").unwrap(),
    };
    let id = store.add_ticket(draft);
    let ticket = store.get(id).unwrap();
    let ticket = ticket.read().unwrap();;
    assert_eq!(ticket.title.to_string(), "Toto".to_string());
}