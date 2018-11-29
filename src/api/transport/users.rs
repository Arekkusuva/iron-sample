use iron::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct PostUser {
    pub name: String,
    pub password: String,
}
