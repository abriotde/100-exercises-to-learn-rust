use tokio::net::TcpListener;
use tokio::net::TcpStream;
// use core::net::SocketAddr;
use crate::store::TicketStore;
use std::fs;

pub async fn serve(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let (mut reader, mut writer) = stream.split();
    println!("new client");
    stream.try_read(&mut buffer).unwrap();
    let root = b"GET / HTTP/1.1\r\n";
    let get = b"GET /get HTTP/1.1\r\n";
    let (status_line, contents) = if buffer.starts_with(root) {
        ("HTTP/1.1 200 OK\r\n\r\n", "{name:'toto', value:'x'}".to_string())
    } else if buffer.starts_with(get) {
        // task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "{name:'toto', value:'y'}".to_string())
    } else {
        println!("Got buffer : {}", String::from_utf8(buffer.to_vec()).unwrap());
        let content = fs::read_to_string("/home/alberic/tmp/error.html").unwrap();
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", content)
    };
    let response = format!("{status_line}{contents}");
    stream.try_write(response.as_bytes()).unwrap();
}

pub async fn run_server() {
    let mut store = TicketStore::new();
    let listener = TcpListener::bind("127.0.0.1:8085").await.unwrap();
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        tokio::spawn(serve(stream));
    }
}