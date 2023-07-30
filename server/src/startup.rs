use std::net::TcpListener;
use crate::server::MyWs;

use actix_web::{dev::Server, HttpServer, App, web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs, &req, stream);
    println!("{:?}", resp);
    resp
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move|| {
        App::new()
        .route("/ws", web::get().to(index))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
