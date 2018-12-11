use iron::prelude::*;
use iron::headers::{Authorization, Bearer};
use jwt;
use r2d2_redis::RedisConnectionManager;
use r2d2_redis::r2d2::Pool;
use r2d2_redis::redis::IntoConnectionInfo;

use api::transport::{Result, RequestError, RequestErrorKind};
use api::middlewares::sessions::stores::redis::{RedisPool, RedisSessionStore};
use api::middlewares::sessions::SessionBackend;

const DEFAULT_SECRET: &'static str = "%4r6gfd453dfSDfdf34!!^63431!33";

pub struct BearerJWTRedisBackend {
    pool: RedisPool,
    secret: String,
}

// TODO: Handle errors.
impl BearerJWTRedisBackend {
    fn build_pool<I: IntoConnectionInfo>(params: I) -> RedisPool {
        let manager = RedisConnectionManager::new(params).unwrap();
        Pool::builder()
            .build(manager)
            .unwrap()
    }

    pub fn new<I: IntoConnectionInfo>(params: I) -> BearerJWTRedisBackend {
        let pool = Self::build_pool(params);

        BearerJWTRedisBackend {
            pool,
            secret: DEFAULT_SECRET.to_string(),
        }
    }

    pub fn with_secret<I: IntoConnectionInfo>(params: I, secret: &str) -> BearerJWTRedisBackend {
        let pool = Self::build_pool(params);

        BearerJWTRedisBackend {
            pool,
            secret: secret.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub user_id: u64,
    pub exp: usize,
}

impl SessionBackend for BearerJWTRedisBackend {
    type Store = RedisSessionStore;

    fn get_store_from_request(&self, request: &mut Request) -> Result<Self::Store> {
        let token = match request.headers.get::<Authorization<Bearer>>() {
            Some(b) => b.token.clone(),
            None => return Err(RequestError::new(RequestErrorKind::Unauthorized)),
        };

        let claims: Claims = match jwt::decode(&token, self.secret.as_bytes(), &jwt::Validation::default()) {
            Ok(t) => t.claims,
            Err(_) => return Err(RequestError::new(RequestErrorKind::Unauthorized)),
        };

        let key_from_token: String = token.rsplitn(2, ".").collect();
        let session_id = format!("{}_{}", claims.user_id, key_from_token);

        Ok(RedisSessionStore {
            session_id,
            pool: self.pool.clone(),
        })
    }
}
