use iron::Handler;

pub mod auth;

pub trait Wrapper {
    fn wrap(handler: Box<Handler>) -> Box<Handler>;
}
