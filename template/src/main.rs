use axel::log::info;
use axel::main;
use axel::startup::start_axel;
use axel::{Axel, ServiceConfig};
use std::io;
use std::sync::Arc;

mod controllers;
mod routes;
mod stores;

pub struct StartupShared;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let startup_shared = StartupShared {};
    let startup_shared = Arc::new(startup_shared);

    start_axel(startup_shared, define_services).await
}

/// Function for defining services called whenever a new
/// app instance is created.
#[inline]
fn define_services(shared: Arc<StartupShared>, cfg: &mut ServiceConfig) {
    // Configure the routing modules
    cfg.configure(routes::hello::axel_configure)
        .configure(routes::test::axel_configure);
}
