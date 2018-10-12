use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;

/// Router contains addresses associated with handlers.
struct Router {
    routes: HashMap<String, Box<Handler>>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route<H>(&mut self, path: String, handler: H)
        where H: Handler
    {
        self.routes.insert(path, Box::new(handler));
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

pub fn start_listening(port: i32) {
    let mut router = Router::new();

    router.add_route("hello".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    });

    Iron::new(router).http(format!("localhost:{}", port)).expect("http");
}
