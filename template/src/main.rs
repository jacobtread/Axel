use axel::prelude::*;

mod routes;
mod stores;

#[actix_web::main]
async fn main() {
    Axel::create(routes::define).await;
}
