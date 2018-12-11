use iron::prelude::*;
use r2d2_redis::RedisConnectionManager;
use r2d2_redis::r2d2::Pool;
use r2d2_redis::redis::IntoConnectionInfo;

use super::super::stores::redis::{RedisPool, RedisSessionStore};
use super::super::SessionBackend;

pub struct BearerJWTRedisBackend {
    pool: RedisPool,
}

// TODO: Handle errors.
impl BearerJWTRedisBackend {
    pub fn new<I: IntoConnectionInfo>(params: I) -> BearerJWTRedisBackend {
        let manager = RedisConnectionManager::new(params).unwrap();
        let pool = Pool::builder()
            .build(manager)
            .unwrap();

        BearerJWTRedisBackend {
            pool
        }
    }
}

impl SessionBackend for BearerJWTRedisBackend {
    type Store = RedisSessionStore;

    fn get_store_from_request(&self, request: &mut Request) -> Self::Store {
    }
}
