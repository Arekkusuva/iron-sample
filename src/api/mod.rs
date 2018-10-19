use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use time::precise_time_ns;
use slog::Logger;

use super::utils::logger::logger_factory;
mod controllers;
use self::controllers::Engage;
mod middlewares;
use self::middlewares::{LoggerMiddleware, ResponseTimeLoggerMiddleware};

// TODO: Add context.
/// Router contains endpoints associated with handlers.
pub struct Router {
    routes: HashMap<String, Box<Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<P, H>(&mut self, path: P, handler: H)
        where P: Into<String>,
              H: Fn(&mut Request) -> IronResult<Response> + Send + Sync + 'static,
    {
        self.add(path, move |req: &mut Request| { handler(req) });
    }

    fn add<P, H>(&mut self, path: P, handler: H)
        where P: Into<String>,
              H: Handler,
    {
        self.routes.insert(path.into(), Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

//#[allow(dead_code)]
//#[derive(Default)]
//struct Context {
//    id: i64,
//}
//
//#[allow(dead_code)]
//impl Context {
//    fn new() -> Self {
//        Context::default()
//    }
//
//    fn set_id(&mut self, id: i64) {
//        self.id = id;
//    }
//}

pub fn start_listening(port: i32) {
    let logger = logger_factory();

    let router= Router::new().engage();
    let mut chain = Chain::new(router);

    // Logger
    let logger_middleware = LoggerMiddleware::new(&logger);
    chain.link_before(logger_middleware);

    // Response time logger
    chain.link_before(ResponseTimeLoggerMiddleware);
    chain.link_after(ResponseTimeLoggerMiddleware);

    Iron::new(chain).http(format!("localhost:{}", port)).expect("http");
}
