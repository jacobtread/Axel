use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug)]
pub struct MyStore {
    names: RwLock<HashMap<String, String>>,
}
