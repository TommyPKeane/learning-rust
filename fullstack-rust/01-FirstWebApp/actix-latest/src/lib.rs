#[macro_use]
extern crate actix_web;

use actix_web::{App, HttpServer, HttpRequest, Responder};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
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

    #[actix_web::main]
    pub async fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: {}:{}", self.address, self.port);

        HttpServer::new(
            move
            || {
                App::new()
                .service(index)
            }
        )
        .bind(format!("{}:{}", self.address, self.port))?
        .workers(4)
        .run()
        .await
    }
}
