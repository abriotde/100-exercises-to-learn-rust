// use ticket_fields::test_helpers::{ticket_description, ticket_title};
use outro_08;
use outro_08::store::TicketStore;
use outro_08::data::TicketDraft;
use outro_08::server;
use ticket_fields::TicketTitle;
use ticket_fields::TicketDescription;
use std::{thread, time};

#[tokio::test]
async fn server_test() {
    // server();
    let move_forward = true;
    // let _ = server::run_server2(30).await;
    tokio::task::spawn(async move {
        let _ = server::run_server2(30).await;
    });
    thread::sleep(time::Duration::from_millis(100000));
    
    let res = reqwest::get("http://127.0.0.1:8085/get?id=1").await.unwrap();
    let body = res.text().await.unwrap();

    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    assert!(move_forward);
}
/*
http://127.0.0.1:8085/get?id=1
-> {error:"No such ticket id."}
http://127.0.0.1:8085/add?title=alberic&description=de%20la%20Crochais
->  {id:"1"}
http://127.0.0.1:8085/get?id=1
-> {"id":1,"title":"alberic","description":"de la Crochais","status":"ToDo"}

http://127.0.0.1:8085/add?draft={%22title%22:%22alberic%22,%20%22description%22:%22de%20la%20Crochais%22}
*/

#[test]
fn store() {
    let store = TicketStore::new();
    let draft = TicketDraft{
        title: TicketTitle::try_from("Toto").unwrap(),
        description: TicketDescription::try_from("il va bien").unwrap(),
    };
    let id = store.add_ticket(draft);
    let ticket = store.get(id).unwrap();
    let ticket = ticket.read().unwrap();;
    assert_eq!(ticket.title.to_string(), "Toto".to_string());
}