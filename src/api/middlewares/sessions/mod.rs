use iron::prelude::*;
use iron::{typemap, AroundMiddleware, Handler};

use api::transport::{Result, RequestError, RequestErrorKind, RequestInternalErrorKind};

pub mod backends;
mod stores;

use self::stores::SessionStore;

pub trait SessionBackend: Send + Sync + 'static {
    type Store: SessionStore;

    fn get_store_from_request(&self, request: &mut Request) -> Result<Self::Store>;
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
}

pub trait SessionValue: Sized + 'static {
    fn get_key() -> &'static str;
    fn into_raw(self) -> String;
    fn from_raw(value: String) -> Option<Self>;
}

impl Session {
    pub fn new(s: Box<SessionStore>) -> Self {
        Session {
            inner: s,
        }
    }

    pub fn set<T: SessionValue>(&mut self, value: T) {
        self.inner.set_raw(T::get_key(), value.into_raw())
    }

    pub fn get<T: SessionValue>(&self) -> Result<T> {
        match self.inner.get_raw(T::get_key()).and_then(T::from_raw) {
            Some(v) => Ok(v),
            None => Err(RequestError::new(RequestErrorKind::InternalError(RequestInternalErrorKind::SessionsStore))),
        }
    }
}

struct SessionKey;
impl typemap::Key for SessionKey { type Value = Session; }

impl<S: SessionBackend> AroundMiddleware for SessionsMiddleware<S> {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(move |req: &mut Request| -> IronResult<Response> {
            match self.backend.get_store_from_request(req) {
                Ok(s) => {
                    req.extensions.insert::<SessionKey>(Session::new(Box::new(s)));
                },
                Err(e) => {
                    req.extensions.insert::<RequestError>(e);
                },
            };
            handler.handle(req)
        })
    }
}

pub trait SessionReqExt {
    fn get_session(&mut self) -> &mut Session;
}

impl<'a, 'b>SessionReqExt for Request<'a, 'b> {
    fn get_session(&mut self) -> &mut Session {
        self.extensions.get_mut::<SessionKey>().unwrap()
    }
}
