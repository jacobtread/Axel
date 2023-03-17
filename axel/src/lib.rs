use crate::services::AxelServices;
use actix_web::{web::ServiceConfig, App, HttpServer};
use log::{error, info};
use std::sync::Arc;

pub mod env;
pub mod macros;
pub mod prelude;
pub mod services;

// Re-export all the actix web functionality
pub use actix_web;

pub use log;

pub struct Axel;

impl Axel {
    pub async fn create<F>(startup: F)
    where
        F: FnOnce(&mut ServiceConfig) + Send + Clone + 'static,
    {
        let this = Self::new();
        let services = this.services();
        let server = HttpServer::new(move || {
            let startup = startup.clone();
            App::new().configure(|cfg| {
                services.configure(cfg);
                startup(cfg);
            })
        });
        let server = match server.bind(Self::get_address()) {
            Ok(server) => server,
            Err(err) => {
                error!("Failed to bind server: {:?}", err);
                panic!();
            }
        };

        match server.run().await {
            Ok(_) => {}
            Err(err) => {
                error!("Fatal error while running server: {:?}", err);
                panic!();
            }
        }
    }

    /// Creates a new instance of Axel. Initialize Axel.
    /// Sets up logging, and environment
    pub(crate) fn new() -> Self {
        env::init();
        env_logger::init();
        info!("Loaded environment variables.. Starting..");
        Self
    }

    /// Returns the socket address made up of the host and port
    /// parsed from the environment variables. This is intended
    /// for use when binding the server. If this either of the
    /// environment variables are missing the defaults will be
    /// used instead (0.0.0.0:80)
    pub(crate) fn get_address() -> (String, u16) {
        const HOST_ENV: &str = "AXEL_HOST";
        const HOST_DEFAULT: &str = "0.0.0.0";

        const PORT_ENV: &str = "AXEL_PORT";
        const DEFAULT_PORT: u16 = 80;

        let host = env::get_env_or(HOST_ENV, HOST_DEFAULT);
        let port = env::parse_env_or(PORT_ENV, DEFAULT_PORT);

        (host, port)
    }

    pub fn services(&self) -> Arc<AxelServices> {
        return AxelServices::new();
    }
}
