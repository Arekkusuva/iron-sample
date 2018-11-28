use iron::prelude::*;
use bodyparser::Struct;
use super::{Result, ParseError, ParseErrorKind};

#[derive(Debug, Clone, Deserialize)]
pub struct PostUser {
    pub name: String,
    pub password: String,
}

impl PostUser {
    pub fn parse_and_validate<'a>(req: &mut Request) -> Result<Self> {
        match req.get::<Struct<PostUser>>() {
            Ok(Some(body)) => Ok(body),
            Ok(None) => Err(ParseError::new(ParseErrorKind::WrongData)),
            Err(_) => Err(ParseError::new(ParseErrorKind::WrongData)),
        }
    }
}