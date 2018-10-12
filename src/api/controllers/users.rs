use iron::prelude::*;
use iron::status;
use super::super::{Router};

fn create(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "create user")))
}

pub fn engage(router: &mut Router) {
    router.add_route("users/create", create);
}
