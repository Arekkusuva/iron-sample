use iron::prelude::*;
use iron::Handler;

use api::transport::RequestError;

pub struct AuthWrapper;

impl AuthWrapper {
    pub fn wrap<H: Handler>(handler: H) -> impl Handler {
        move |req: &mut Request| {
            match req.extensions.remove::<RequestError>() {
                Some(e) => Ok(Response::from(e)),
                None => handler.handle(req),
            }
        }
    }
}
