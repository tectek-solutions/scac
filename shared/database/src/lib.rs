pub mod model;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl Database {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Self {
            pool: Arc::new(pool),
        }
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool
            .get()
            .expect("Failed to get a connection from the pool.")
    }
}

