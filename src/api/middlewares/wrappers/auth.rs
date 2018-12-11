use iron::prelude::*;
use iron::Handler;

use api::transport::{RequestError, RequestErrorKind};
use super::Wrapper;

pub struct AuthWrapper;

impl Wrapper for AuthWrapper {
    fn wrap(handler: Box<Handler>) -> Box<Handler> {
        Box::new(move |req: &mut Request| {
            match req.extensions.remove::<RequestError>() {
                Some(e) => Ok(Response::from(e)),
                None => handler.handle(req),
            }
        })
    }
}
