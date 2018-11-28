use std::collections::HashMap;
use std::collections::hash_map::Entry;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use time::precise_time_ns;
use slog::Logger;
use persistent;
use bodyparser;

use super::utils::logger::logger_factory;
mod controllers;
use self::controllers::Engage;
mod middlewares;
use self::middlewares::{LoggerMiddleware, ResponseTimeLoggerMiddleware};
mod deserializer;

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

    pub fn get_full_path<P, M>(path: P, method: M) -> String
        where P: Into<String>,
              M: Into<String>,
    {
        format!("{}-{}", method.into(), path.into())
    }

    fn add_route<P, M, H>(&mut self, path: P, method: M, handler: H)
        where P: Into<String>,
              M: Into<String>,
              H: Handler,
    {
        let mut key = Router::get_full_path(path, method);
        self.routes.insert(key, Box::new(handler));
    }

    pub fn get<P, H>(&mut self, path: P, handler: H)
        where P: Into<String>,
              H: Fn(&mut Request) -> IronResult<Response> + Send + Sync + 'static,
    {
        self.add_route(path, "GET", move |req: &mut Request| { handler(req) });
    }

    pub fn post<P, H>(&mut self, path: P, handler: H)
        where P: Into<String>,
              H: Fn(&mut Request) -> IronResult<Response> + Send + Sync + 'static,
    {
        self.add_route(path, "POST", move |req: &mut Request| { handler(req) });
    }

}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&Router::get_full_path(req.url.path().join("/"), req.method.as_ref())) {
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

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

pub fn start_listening(port: i32) {
    // Create root logger
    let logger = logger_factory();

    // Create router
    let router= Router::new().engage();
    let mut chain = Chain::new(router);

    // Body limit
    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));

    // Logger
    let logger_middleware = LoggerMiddleware::new(&logger);
    chain.link_before(logger_middleware);

    // Response time logging middleware
    chain.link_before(ResponseTimeLoggerMiddleware);
    chain.link_after(ResponseTimeLoggerMiddleware);

    // Start listening on specific port
    Iron::new(chain).http(format!("localhost:{}", port)).expect("http");
}
