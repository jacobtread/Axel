use actix_web::Responder;
use axel::controller::Controller;
use axel::get;
use axel_derive::Controller;

/// Controllers are empty classes
#[derive(Controller)]
#[scope("/test")]
pub struct TestController;

impl TestController {
    #[get("abc")]
    pub async fn test() -> impl Responder {
        "This is abc"
    }
}
