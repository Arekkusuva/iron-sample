use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};
use slog::Logger;

pub struct LoggerMiddleware {
    pub logger: Logger,
}

impl LoggerMiddleware {
    pub fn new(logger: &Logger) -> LoggerMiddleware {
        LoggerMiddleware {
            logger: logger.new(o!("module" => "LoggerMiddleware"))
        }
    }
}

pub struct Value(Logger);

impl typemap::Key for LoggerMiddleware { type Value = Value; }

impl BeforeMiddleware for LoggerMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let logger = self.logger.new(o!("route" => format!("{}", req.url)));
        req.extensions.insert::<LoggerMiddleware>(Value(logger));
        Ok(())
    }
}

pub trait LoggerReqExt {
    fn get_logger(&self) -> Logger;
}

impl<'a, 'b>LoggerReqExt for Request<'a, 'b> {
    fn get_logger(&self) -> Logger {
        let &Value(ref logger) = self.extensions.get::<LoggerMiddleware>().expect("Failed to get logger");

        logger.clone()
    }
}
