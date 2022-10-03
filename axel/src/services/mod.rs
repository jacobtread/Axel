use std::sync::Arc;

/// Stucture containing services that Axel provides this is
/// thread safe and should be configured on all App instances
pub struct AxelServices {}

impl AxelServices {
    pub(crate) fn new() -> Arc<AxelServices> {
        Arc::new(Self {})
    }

    /// Configures the provided service config applying the
    /// configuration for Axel
    pub fn configure(&self, cfg: &mut actix_web::web::ServiceConfig) {}
}
