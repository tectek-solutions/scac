use r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;

#[derive(Clone)]
pub struct Cache {
    pool: Pool<RedisConnectionManager>,
}

impl Cache {
    pub fn new(cache_url: &str) -> Self {
        let manager = RedisConnectionManager::new(cache_url).unwrap();
        let pool = Pool::builder().build(manager).unwrap();
        Cache { pool }
    }

    pub fn get_connection(&self) -> PooledConnection<RedisConnectionManager> {
        self.pool
            .get()
            .expect("Failed to get a connection from the pool.")
    }
}