use actix_web::web::ServiceConfig;

pub trait Controller {
    /// Function for configuring the controller to add
    /// it self to the application
    fn configure(cfg: &mut ServiceConfig);
}
