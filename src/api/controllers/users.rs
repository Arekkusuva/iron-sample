use std::error::Error;

use iron::prelude::*;
use iron::status;
use super::super::Router;

use api::deserializer::users::{PostUser};

fn post_user(req: &mut Request) -> IronResult<Response> {
    let body = match PostUser::parse_and_validate(req) {
        Ok(body) => body,
        Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
    };

    Ok(Response::with((status::Ok, body.name)))
}

fn get_users(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "get users")))
}

pub fn engage(router: &mut Router) {
    router.post("users/new", post_user);
    router.get("users", get_users);
}
