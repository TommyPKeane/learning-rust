#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

static SERVER_COUNTER:AtomicUsize = AtomicUsize::new(0);

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Deserialize)]
struct PostInput {
    message: String,
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    message: Vec<String>,
}

#[get("/")]
fn index(state:web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();

    Result::Ok(
        web::Json(
            IndexResponse {
                server_id: state.server_id,
                request_count,
                message: ms.clone(),
            }
        )
    )
}


fn post(msg:web::Json<PostInput>, state:web::Data<AppState>) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut ms = state.messages.lock().unwrap();
    ms.push(msg.message.clone());

    Ok(
        web::Json(
            PostResponse {
                server_id: state.server_id,
                request_count: request_count,
                message: msg.message.clone(),
            }
        )
    )
}

#[post("/clear")]
fn clear(state:web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut ms = state.messages.lock().unwrap();
    ms.clear();

    Ok(
        web::Json(
            IndexResponse {
                server_id: state.server_id,
                request_count: request_count,
                message: vec![],
            }
        )
    )
}

pub struct MessageApp {
    port: u16,
    address: String,
}

impl MessageApp {
    pub fn new(address: &str, port: u16) -> Self {
        MessageApp {
            port: port,
            address: address.to_string(),
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let messages = Arc::new(Mutex::new(vec![]));
        println!("Starting http server: {}:{}", self.address, self.port);

        HttpServer::new(
            move
            || {
                App::new()
                .data(
                    AppState {
                        server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                        request_count: Cell::new(0),
                        messages: messages.clone(),
                    }
                )
                .wrap(middleware::Logger::default())
                .service(index)
                .service(
                    web::resource("/send")
                    .data(web::JsonConfig::default().limit(4096))
                    .route(web::post().to(post)),
                )
                .service(clear)
            }
        )
        .bind(format!("{}:{}", self.address, self.port))?
        .workers(8)
        .run()
    }
}
