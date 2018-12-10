use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::{Commands};

use super::SessionStore;

pub struct RedisSessionStore {
    pool: r2d2::Pool<RedisConnectionManager>,
}

impl RedisSessionStore {
    fn new(uri: &str) -> RedisSessionStore {
        let manager = RedisConnectionManager::new(uri).expect("Failed create redis manager");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed build redis connection pool");

        RedisSessionStore {
            pool
        }
    }
}

impl SessionStore for RedisSessionStore {
    fn set_string(&self, key: &str, value: String) {
        let conn = self.pool.get().unwrap();
        let _: () = conn.set(key, value).unwrap();
    }

    fn get_string(&self, key: &str) -> Option<String> {
        let conn = self.pool.get().unwrap();
        conn.get(key).unwrap()
    }
}
