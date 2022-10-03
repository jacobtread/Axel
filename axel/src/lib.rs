use crate::services::AxelServices;
use actix_web::HttpServer;
use log::info;
use std::net::ToSocketAddrs;
use std::sync::Arc;

pub mod controller;
pub mod env;
pub mod macros;
pub mod services;
pub mod startup;

pub extern crate actix_web;

// Re-export all the actix web functionality
pub use actix_web::web::*;
pub use actix_web::*;

pub use log;

pub struct Axel;

impl Axel {
    /// Creates a new instance of Axel. Initialize Axel.
    /// Sets up logging, and environment
    pub fn new() -> Self {
        env::init();
        env_logger::init();
        info!("Loaded environment variables. Starting..");

        Self
    }

    /// Returns the socket address made up of the host and port
    /// parsed from the environment variables. This is intended
    /// for use when binding the server. If this either of the
    /// environment variables are missing the defaults will be
    /// used instead (0.0.0.0:80)
    pub fn get_address(&self) -> (String, u16) {
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
