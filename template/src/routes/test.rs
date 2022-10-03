use axel::actix_web::{get, Responder};
use axel::define_routes;

// Define the routes that are present in this file
define_routes![test];

/// Simple function which responds to all requests
/// at GET /test with "This is a test"
#[get("/test")]
pub async fn test() -> impl Responder {
    "This is a test"
}
