use std::error::Error;
use std::fmt;
use std::result::Result as StdResult;

use iron::prelude::{Request, Plugin};
use serde::Deserialize;
use bodyparser::Struct;
use validator::Validate;

pub mod prelude;
pub mod users;

type Result<T> = StdResult<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    WrongData,
    NotValid(String),
}

impl ParseError {
    pub fn new(kind: ParseErrorKind) -> ParseError {
        ParseError { kind }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self.kind {
            ParseErrorKind::WrongData => "invalid data",
            ParseErrorKind::NotValid(ref err_msg) => err_msg,
        }
    }
}

pub trait BodyParser {
    fn parse_body<T>(&mut self) -> Result<T>
        where T: for<'c> Deserialize<'c> + Clone + Validate + 'static;
}

impl<'a, 'b> BodyParser for Request<'a, 'b> {
    fn parse_body<T>(&mut self) -> Result<T>
        where T: for<'c> Deserialize<'c> + Clone + Validate + 'static,
    {
        match self.get::<Struct<T>>() {
            Ok(Some(body)) => match body.validate() {
                Ok(_) => Ok(body),
                Err(err) => Err(ParseError::new(ParseErrorKind::NotValid(err.description().to_string()))),
            },
            Ok(None) => Err(ParseError::new(ParseErrorKind::WrongData)),
            Err(_) => Err(ParseError::new(ParseErrorKind::WrongData)),
        }
    }
}
