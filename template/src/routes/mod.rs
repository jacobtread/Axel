use crate::StartupData;
use axel::actix_web::web::{scope, ServiceConfig};
use std::sync::Arc;

mod hello;
mod scoped;
mod test;

/// Function for defining routes
pub fn define(_shared: Arc<StartupData>, cfg: &mut ServiceConfig) {
    // Configure the routing modules
    cfg.configure(hello::axel_configure)
        .configure(test::axel_configure)
        .service(scope("scoped").configure(scoped::axel_configure));
}
