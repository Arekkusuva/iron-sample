use std::env;

use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};
use diesel::pg::PgConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager, Pool};
use slog::Logger;

pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub struct DBMiddleware {
    pool: DBPool,
}

impl DBMiddleware {
    pub fn new(logger: &Logger) -> DBMiddleware {
        let logger = logger.new(o!("module" => "DBMiddleware"));

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::new(manager).expect("Failed to create diesel pool");

        info!(logger, "DB pool created");

        DBMiddleware { pool }
    }
}

pub struct Value(DBPool);

impl typemap::Key for DBMiddleware { type Value = Value; }

impl BeforeMiddleware for DBMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DBMiddleware>(Value(self.pool.clone()));
        Ok(())
    }
}

pub trait DBRequestExt {
    fn get_db_conn(&self) -> DBConnection;
}

impl<'a, 'b> DBRequestExt for Request<'a, 'b> {
    fn get_db_conn(&self) -> DBConnection {
        let &Value(ref pool) = self.extensions.get::<DBMiddleware>().expect("Failed to get db");

        pool.get().expect("Failed to get a db connection")
    }
}
