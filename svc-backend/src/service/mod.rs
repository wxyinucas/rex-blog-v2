#![allow(clippy::all, unused_imports, dead_code)]
use std::sync::Arc;

use crate::storage::DBPool;

pub mod implements;

#[cfg(test)]
mod tests;

pub struct BackendInnerService {
    db_pool: Arc<DBPool>,
}

impl BackendInnerService {
    pub fn new(db_pool: DBPool) -> Self {
        Self {
            db_pool: Arc::new(db_pool),
        }
    }
}

impl Clone for BackendInnerService {
    fn clone(&self) -> Self {
        Self {
            db_pool: Arc::clone(&self.db_pool),
        }
    }
}
