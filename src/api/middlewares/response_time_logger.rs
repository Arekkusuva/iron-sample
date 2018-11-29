use iron::{typemap, BeforeMiddleware, AfterMiddleware};
use iron::prelude::*;
use time::precise_time_ns;
use api::middlewares::logger::LoggerReqExt;

pub struct ResponseTimeLoggerMiddleware;

impl typemap::Key for ResponseTimeLoggerMiddleware { type Value = u64; }

impl BeforeMiddleware for ResponseTimeLoggerMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTimeLoggerMiddleware>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTimeLoggerMiddleware {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTimeLoggerMiddleware>().unwrap();
        let logger = req.get_logger();
        info!(logger, "Request took: {} ms", delta as f64 / 1000000.0);
        Ok(res)
    }
}
