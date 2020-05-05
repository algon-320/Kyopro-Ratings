use chrono::prelude::*;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug)]
pub struct FreshCache<T: Clone> {
    hash: RwLock<HashMap<String, RwLock<(T, DateTime<Utc>)>>>,
}

impl<T: Clone> FreshCache<T> {
    pub fn new() -> Self {
        Self {
            hash: RwLock::new(HashMap::new()),
        }
    }
    pub fn get(&self, key: &str) -> Option<T> {
        let tbl_reader = self.hash.read().unwrap();
        let entry = tbl_reader.get(key)?.read().unwrap();
        let (v, deadline) = entry.clone();
        if deadline > Utc::now() {
            Some(v)
        } else {
            None
        }
    }
    pub fn store(&self, key: impl Into<String>, value: T, limit: DateTime<Utc>) {
        let mut tbl_writer = self.hash.write().unwrap();
        tbl_writer.insert(key.into(), RwLock::new((value, limit)));
    }
}
