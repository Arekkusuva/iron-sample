use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use time::precise_time_ns;

mod controllers;
use self::controllers::Engage;

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

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

#[allow(dead_code)]
#[derive(Default)]
struct Context {
    id: i64,
}

#[allow(dead_code)]
impl Context {
    fn new() -> Self {
        Context::default()
    }

    fn set_id(&mut self, id: i64) {
        self.id = id;
    }
}

pub fn start_listening(port: i32) {
    let router= Router::new().engage();

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);

    Iron::new(chain).http(format!("localhost:{}", port)).expect("http");
}
