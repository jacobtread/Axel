use axel::store::Store;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MyStore {
    names: RwLock<HashMap<String, String>>,
}
