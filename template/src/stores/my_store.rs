use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug)]
pub struct MyStore {
    pub names: RwLock<HashMap<String, String>>,
}
