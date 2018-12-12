use std::error::Error;
use std::fmt;
use std::result::Result as StdResult;
use std::collections::HashMap;
use std::convert::{Into, From};

use iron::prelude::*;
use iron::{typemap, status};
use serde::{Deserialize, Serialize};
use bodyparser::Struct;
use serde_json::Value;
use validator::{ValidationErrors, Validate};
use ijr::JsonResponse;

use api::utils;

pub mod prelude;
pub mod users;

pub type Result<T> = StdResult<T, RequestError>;

#[derive(Debug, Clone, Serialize)]
pub struct ResponseBody {
    #[serde(skip_serializing)]
    pub resp_status: status::Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_code: Option<u8>,
}

impl ResponseBody {
    pub fn with_data<T: Serialize>(resp_status: status::Status, data: T) -> ResponseBody {
        ResponseBody {
            resp_status,
            error: None,
            data: Some(Box::new(json!(data))),
            internal_code: None,
        }
    }

    pub fn with_error(err: RequestError) -> ResponseBody {
        match err.kind {
            RequestErrorKind::WrongData => {
                ResponseBody {
                    resp_status: status::BadRequest,
                    error: Some("wrong_data"),
                    data: None,
                    internal_code: None,
                }
            },
            RequestErrorKind::NotValid(e) => {
                let fields = e.field_errors();
                let mut not_valid_fields: HashMap<String, &'static str> = HashMap::with_capacity(fields.len());
                for (f, _) in fields {
                    not_valid_fields.insert(
                        f.to_string(),
                        utils::get_validation_message(f),
                    );
                }
                ResponseBody {
                    resp_status: status::BadRequest,
                    error: Some("validation_failed"),
                    data: Some(Box::new(json!(not_valid_fields))),
                    internal_code: None,
                }
            },
            RequestErrorKind::Unauthorized => {
                ResponseBody {
                    resp_status: status::Unauthorized,
                    error: status::Unauthorized.canonical_reason(),
                    data: None,
                    internal_code: None,
                }
            },
            RequestErrorKind::InternalError(e) => {
                ResponseBody {
                    resp_status: status::InternalServerError,
                    error: status::InternalServerError.canonical_reason(),
                    data: None,
                    internal_code: Some(e as u8),
                }
            }
        }
    }
}

impl From<ResponseBody> for Response {
    fn from(body: ResponseBody) -> Response {
        let mut resp = Response::new();
        resp.set_mut(body.resp_status)
            .set_mut(JsonResponse::json(body));
        resp
    }
}

#[derive(Debug, Clone)]
pub struct RequestError {
    pub kind: RequestErrorKind
}

#[derive(Debug, Clone)]
pub enum RequestErrorKind {
    WrongData,
    NotValid(Box<ValidationErrors>),
    Unauthorized,
    InternalError(RequestInternalErrorKind),
}

#[derive(Debug, Clone)]
pub enum RequestInternalErrorKind {
    SessionsStore
}

impl RequestError {
    pub fn new(kind: RequestErrorKind) -> RequestError {
        RequestError { kind }
    }
}

impl typemap::Key for RequestError { type Value = RequestError; }

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for RequestError {
    fn description(&self) -> &str {
        match self.kind {
            RequestErrorKind::WrongData => "Parsing failed",
            RequestErrorKind::NotValid(_) => "Validation failed",
            RequestErrorKind::Unauthorized => "Unauthorized",
            RequestErrorKind::InternalError(_) => "Internal error",
        }
    }
}

impl From<RequestError> for Response {
    fn from(err: RequestError) -> Response {
        let resp_body = ResponseBody::with_error(err);
        resp_body.into()
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
                Err(err) => Err(RequestError::new(RequestErrorKind::NotValid(Box::new(err)))),
            },
            Ok(None) => Err(RequestError::new(RequestErrorKind::WrongData)),
            Err(_) => Err(RequestError::new(RequestErrorKind::WrongData)),
        }
    }
}
