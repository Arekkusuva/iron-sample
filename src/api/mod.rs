use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use ijr::{JsonResponseMiddleware};

use super::utils::logger::logger_factory;
mod controllers;
use self::controllers::Engage;
mod middlewares;
use self::middlewares::{LoggerMiddleware, ResponseTimeLoggerMiddleware};
mod transport;
mod utils;

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
        let key = Router::get_full_path(path, method);
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

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

pub fn start_listening(port: i32) {
    // Create root logger
    let logger = logger_factory();

    // Create router
    let router= Router::new().engage();
    let mut chain = Chain::new(router);

    // Link middlewares
    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    let logger_middleware = LoggerMiddleware::new(&logger);
    chain.link_before(logger_middleware);
    chain.link_after(JsonResponseMiddleware::new());
    chain.link_before(ResponseTimeLoggerMiddleware);
    chain.link_after(ResponseTimeLoggerMiddleware);

    // Start listening on specific port
    Iron::new(chain).http(format!("localhost:{}", port)).expect("http");
}
