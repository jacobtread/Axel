/// Macro for defining a "axel_configure" function which
/// adds the provided routes as services on the application
///
/// Using this macro:
/// ```
/// use axel::define_routes;
/// define_routes![example, test];
/// ```
#[macro_export]
macro_rules! define_routes {
    (
        $($route:ident),*
    ) => {
        pub fn axel_configure(cfg: &mut actix_web::web::ServiceConfig) {
            cfg
                $(.service($route))*;
        }
    };
}
