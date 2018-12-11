use iron::prelude::*;
use iron::{typemap, BeforeMiddleware, Handler};
use slog::Logger;

pub mod backends;
mod stores;

use self::stores::SessionStore;

pub trait SessionBackend: Send + Sync + 'static {
    type Store: SessionStore;

    fn get_store_from_request(&self, request: &mut Request) -> Self::Store;
}

pub struct SessionsMiddleware<S: SessionBackend> {
    backend: S,
}

impl<S: SessionBackend> SessionsMiddleware<S> {
    pub fn new(backend: S) -> SessionsMiddleware<S> {
        SessionsMiddleware { backend }
    }
}

pub struct Session {
    inner: Box<SessionStore>,
    has_changed: bool,
}

impl Session {
    fn new(s: Box<SessionStore>) -> Self {
        Session {
            inner: s,
            has_changed: false,
        }
    }
}

struct SessionKey;
impl typemap::Key for SessionKey { type Value = Session; }

// TODO: Use AroundMiddleware
impl<S: SessionBackend> BeforeMiddleware for SessionsMiddleware<S> {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let s = self.backend.get_store_from_request(req);
        req.extensions.insert::<SessionKey>(Session::new(Box::new(s)));
        Ok(())
    }
}

pub trait SessionReqExt {
    fn get_session(&mut self) -> &mut Session;
}

impl<'a, 'b>SessionReqExt for Request<'a, 'b> {
    fn get_session(&mut self) -> &mut Session {
        self.extensions.get_mut::<SessionKey>().expect("Failed to get session")
    }
}
