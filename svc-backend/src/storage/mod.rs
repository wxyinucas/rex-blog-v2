#![allow(clippy::all, unused_imports, dead_code)]

use std::ops::Deref;

use sqlx::PgPool;

pub use traits::{BlogDB, ID};

mod implements;
mod traits;

#[cfg(test)]
mod tests;

pub struct DBPool {
    pool: PgPool,
}

impl DBPool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Deref for DBPool {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}
