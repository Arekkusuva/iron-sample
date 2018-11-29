use std::clone::Clone;

use iron::prelude::*;
use bodyparser::Struct;
use super::{Result, ParseError, ParseErrorKind};
use serde::Deserialize;

pub trait BodyParser {
    fn parse_body<T>(&mut self) -> Result<T>
        where T: for<'c> Deserialize<'c> + Clone + 'static;
}

impl<'a, 'b> BodyParser for Request<'a, 'b> {
    fn parse_body<T>(&mut self) -> Result<T>
        where T: for<'c> Deserialize<'c> + Clone + 'static,
    {
        match self.get::<Struct<T>>() {
            Ok(Some(body)) => Ok(body),
            Ok(None) => Err(ParseError::new(ParseErrorKind::WrongData)),
            Err(_) => Err(ParseError::new(ParseErrorKind::WrongData)),
        }
    }
}

