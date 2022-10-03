use crate::Axel;
use actix_web::web::ServiceConfig;
use actix_web::{App, HttpServer};
use std::io;
use std::sync::Arc;

/// Starts up the axel server with the provided startup state.
/// Startup state is provided to the setup function
pub async fn start_axel<T: Send + Sync + 'static>(
    startup: Arc<T>,
    startup_fn: fn(Arc<T>, &mut ServiceConfig),
) -> io::Result<()> {
    // Create axel instance
    let axel = Axel::new();
    let services = axel.services();
    let server = HttpServer::new(move || {
        let startup_shared = Arc::clone(&startup);
        App::new().configure(|cfg| {
            services.configure(cfg);
            startup_fn(startup_shared, cfg);
        })
    });

    server.bind(axel.get_address())?.run().await
}
