use actix_web::{App, HttpServer};
use axel::Axel;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let axel = Axel::new();
    let services = axel.services();

    let server = HttpServer::new(move || {
        let services = services.clone();

        // Configure the app with axel
        App::new().configure(|cfg| services.configure(cfg))
    });

    server.bind(axel.get_address())?.run().await
}
