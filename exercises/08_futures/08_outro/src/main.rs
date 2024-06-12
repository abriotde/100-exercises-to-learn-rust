use outro_08::server;

pub use async_attributes::{main, test};
// use actix_web::{HttpServer, App};

#[tokio::main]
async fn  main()  -> std::io::Result<()> {
    server::run_server2().await;
    Ok(())
}