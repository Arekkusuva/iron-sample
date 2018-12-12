use iron::prelude::*;
use iron::status;

use api::transport::prelude::*;
use api::middlewares::prelude::*;
use api::Router;
use api::transport::users::PostUser;
use api::middlewares::wrappers::AuthWrapper;

fn post_user(req: &mut Request) -> IronResult<Response> {
    let body: PostUser = match req.parse_body() {
        Ok(body) => body,
        Err(err) => return Ok(err.into()),
    };
    Ok(Response::with((status::Ok, body.email)))
}

fn get_users(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "get users")))
}

pub fn engage(router: &mut Router) {
    router.post("users/new", post_user);
    router.get("users", AuthWrapper::wrap(get_users));
}
