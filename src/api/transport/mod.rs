use std::error::Error;
use std::fmt;
use std::result::Result as StdResult;
use std::collections::HashMap;
use std::convert::Into;

use iron::prelude::*;
use iron::status;
use serde::{Deserialize, Serialize};
use bodyparser::Struct;
use serde_json::Value;
use validator::{ValidationErrors, Validate};
use ijr::JsonResponse;

use api::utils;

pub mod prelude;
pub mod users;

type Result<T> = StdResult<T, RequestError>;

#[derive(Debug, Clone, Serialize)]
pub struct ResponseBody {
    #[serde(skip_serializing)]
    pub resp_status: status::Status,
    pub error: Option<&'static str>,
    pub data: Option<Box<Value>>,
}

impl ResponseBody {
    pub fn with_data<T>(resp_status: status::Status, data: T) -> ResponseBody
        where T: Serialize,
    {
        ResponseBody {
            resp_status,
            error: None,
            data: Some(Box::new(json!(data))),
        }
    }

    pub fn with_error(err: RequestError) -> ResponseBody {
        match err.kind {
            RequestErrorKind::WrongData => {
                ResponseBody {
                    resp_status: status::BadRequest,
                    error: Some("wrong_data"),
                    data: None,
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
                }
            },
        }
    }
}

impl Into<Response> for ResponseBody {
    fn into(self) -> Response {
        let mut resp = Response::new();
        resp.set_mut(self.resp_status)
            .set_mut(JsonResponse::json(self));
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
}

impl RequestError {
    pub fn new(kind: RequestErrorKind) -> RequestError {
        RequestError { kind }
    }
}

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
        }
    }
}

impl Into<Response> for RequestError {
    fn into(self) -> Response {
        let resp_body = ResponseBody::with_error(self);
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
