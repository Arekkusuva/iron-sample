use iron::Handler;

mod auth;

pub use self::auth::AuthWrapper;

pub trait Wrapper {
    fn wrap<H: Handler>(handler: H) -> Box<Handler>;
}
