use axel::{define_routes, get, Responder};

// Define the routes that are present in this file
define_routes![hello_world];

/// Simple function which responds to all requests
/// at GET /hello with "Hello World"
#[get("/hello")]
pub async fn hello_world() -> impl Responder {
    "Hello World"
}
