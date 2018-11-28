use std::error::Error;
use std::fmt;
use std::result::Result as StdResult;

pub mod users;

type Result<T> = StdResult<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError {
    kind: ParseErrorKind
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    WrongData,
    NotValid(String),
}

impl ParseError {
    fn new(kind: ParseErrorKind) -> ParseError {
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
            ParseErrorKind::WrongData => "failed to parse data",
            ParseErrorKind::NotValid(ref err_msg) => err_msg,
        }
    }
}
