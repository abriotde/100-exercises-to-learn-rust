
use std::time::SystemTime;
use std::collections::HashMap;
use std::fs;
use std::net::SocketAddr;
use std::pin::Pin;
use std::future::Future;
use http_body_util::Full;
use std::io::{Error, ErrorKind};

use tokio::net::TcpListener;
use tokio::net::TcpStream;

use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
// use hyper::service::service_fn;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response, Method, StatusCode};

use crate::data::{TicketId, TicketJson, TicketDraft};
use crate::store::TicketStore;
use ticket_fields::{TicketDescription, TicketTitle};


// ____ Run server using only tokio API ____

pub async fn serve(stream: TcpStream) {
    let mut buffer = [0; 1024];
    // let (mut reader, mut writer) = stream.split();
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
/**
    Run server using only tokio API
*/
pub async fn run_server() {
    // let mut store = TicketStore::new();
    let listener = TcpListener::bind("127.0.0.1:8085").await.unwrap();
    loop {
        let ( stream, _) = listener.accept().await.unwrap();
        tokio::spawn(serve(stream));
    }
}

// ____ Run server using Hyper API ____

// #[async_trait]
impl Service<Request<IncomingBody>> for TicketStore {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }
        fn send_response(s: Response<Full<Bytes>>) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(s)
        }
        fn stop_response() -> Result<Response<Full<Bytes>>, Error> {
            Err(Error::new(ErrorKind::Other, "STOP"))
        }
        fn full<T: Into<Bytes>>(chunk: T) -> Full<Bytes> {
            Full::new(chunk.into())
        }
        // https://github.com/hyperium/hyper/blob/master/examples/params.rs
        // let body = req.body();
        let mut ticket_id: Option<u64> = None;
        let mut ticket: Option<TicketJson> = None;
        let mut title: Option<String> = None;
        let mut description = "".to_string();
        match req.method() {
            &Method::GET =>  {
                if let Some(q) = req.uri().query() {
                    let query = q;
                    let params = form_urlencoded::parse(query.as_bytes())
                        .into_owned()
                        .collect::<HashMap<String, String>>();
                    if let Some(id) = params.get("id") {
                        if let Ok(i) = id.parse::<u64>() {
                            println!("ticket_id = {}", i);
                            ticket_id = Some(i);
                        }
                    };
                    if let Some(t) = params.get("title") {
                        println!("title = {}", t);
                        title = Some(t.to_string());
                    };
                    if let Some(descr) = params.get("description") {
                        println!("description = {}", descr);
                        description = descr.to_string();
                    };
                    if let Some(json) = params.get("ticket") {
                        println!("ticket = {}", json);
                        let ticket_res: serde_json::Result<TicketJson> = serde_json::from_str(json);
                        match ticket_res {
                            Ok(draft) => {
                                ticket = Some(draft)
                            },
                            Err(e) => {
                                println!("Fail parse to JSON ticket : {}", e)
                            }
                        }
                    };
                }
            }
            &Method::POST =>  {
                // let mut files = multipart::server::Multipart::from(req);
                let stream = req.body();
                // let b = stream.into_data_stream();
                println!("Body : {:?}", stream);
                /* let params = form_urlencoded::parse(sream.as_ref())
                    .into_owned()
                    .collect::<HashMap<String, String>>();
                    */
            }
            _ => {}
        };
        let res = match req.uri().path() {
            "/" => "Hello world".to_string(),
            "/get" => {
                match ticket_id {
                    Some(id) => {
                        match self.get(TicketId::from(id)) {
                            Some(ticket) => {
                                let ticket = ticket.read().unwrap();
                                let json_ticket = TicketJson::new(ticket.clone());
                                serde_json::to_string(&json_ticket).unwrap()
                            },
                            _ => {
                                "{error:\"No such ticket id.\"}".to_string()
                            }
                        }
                    },
                    _ => {
                        "{error:\"Missing parameter ticket id : 'id'.\"}".to_string()
                    }
                }
            }
            "/add" => {
                match title {
                    Some(t) => {
                        let title = TicketTitle::try_from(t).unwrap();
                        let description = TicketDescription::try_from(description).unwrap();
                        let draft = TicketDraft{
                            title: title,
                            description: description,
                        };
                        let id = self.add_ticket(draft);
                        format!("{{id:\"{}\"}}", id)
                    }
                    _ => {
                        "{error:\"Missing parameter 'draft'.\"}".to_string()
                    }
                 }
            },
            "/stop" => {
                // return Box::pin(async { stop_response() });
                "STOP".into()
            },
            _ => "404 Error".into(),
        };
        Box::pin(async { mk_response(res) })
    }
}
/**
    Run server using Hyper API
*/
pub async fn run_server2(timeout:u64) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8085));
    let listener = TcpListener::bind(addr).await?;
    let store = TicketStore::new();
    let start = SystemTime::now();
    loop {
        println!("next loop");
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let store_clone = store.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, store_clone)
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
        if timeout>0 {
            println!("Timeout : {}", timeout);
            let duration = SystemTime::now().duration_since(start).unwrap();
            println!("Timeout : {} VS {}", timeout, duration.as_secs());
            if duration.as_secs()>timeout {
                println!("Time to quit : {}", timeout);
                return Err(Box::new(Error::new(ErrorKind::Other, "Timeout")));
            }
        }
    }
}
