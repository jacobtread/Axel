use crate::stores::my_store::MyStore;
use actix_web::web::{Data, ServiceConfig};
use axel::startup::start_axel;
use axel::Axel;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, RwLock};

mod routes;
mod stores;

pub struct StartupData {
    my_store: Data<MyStore>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let store = MyStore {
        names: RwLock::new(HashMap::new()),
    };
    let startup_shared = StartupData {
        my_store: Data::new(store),
    };
    let startup = Arc::new(startup_shared);

    start_axel(startup, |startup, cfg| {
        // Configure the routing modules
        routes::define(startup, cfg);
    })
    .await
}
