use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::{Commands};

use super::SessionStore;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

pub struct RedisSessionStore {
    pub session_id: String,
    pub pool: RedisPool,
}

impl SessionStore for RedisSessionStore {
    fn set_raw(&self, key: &str, value: String) {
        let conn = self.pool.get().unwrap();
        let _: () = conn.hset(&self.session_id, key, value).unwrap();
    }

    fn get_raw(&self, key: &str) -> Option<String> {
        let conn = self.pool.get().unwrap();
        conn.hget(&self.session_id, key).unwrap()
    }
}
