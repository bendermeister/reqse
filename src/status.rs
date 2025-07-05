use std::{fmt::Display, str::FromStr};

use crate::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    // 2xx success codes
    Ok,

    // 3xx redirection
    MultipleChoices,

    // 4xx client error
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    IamATeapot,

    // 5xx server error
    InternalServerError,
    NotImplemented,
    ServiceUnavailable,
    HttpVersionNotSupported,
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, _) = s.trim().split_once(" ").ok_or(Error::InvalidHeader)?;

        match num {
            "200" => return Ok(Self::Ok),
            "300" => return Ok(Self::MultipleChoices),
            "400" => return Ok(Self::BadRequest),
            "401" => return Ok(Self::Unauthorized),
            "403" => return Ok(Self::Forbidden),
            "404" => return Ok(Self::NotFound),
            "405" => return Ok(Self::MethodNotAllowed),
            "418" => return Ok(Self::IamATeapot),
            "500" => return Ok(Self::InternalServerError),
            "503" => return Ok(Self::ServiceUnavailable),
            "505" => return Ok(Self::HttpVersionNotSupported),
            _ => (),
        }

        todo!()
    }
}

impl Status {
    pub fn to_static_str(&self) -> &'static str {
        match self {
            Status::Ok => "200 OK",
            Status::MultipleChoices => "300 Multiple Choices",
            Status::BadRequest => "400 Bad Request",
            Status::Unauthorized => "401 Unauthorized",
            Status::Forbidden => "403 Forbidden",
            Status::NotFound => "404 Not Found",
            Status::MethodNotAllowed => "405 Method Not Allowed",
            Status::IamATeapot => "418 Im a teapot",
            Status::InternalServerError => "500 Internal Server Error",
            Status::NotImplemented => "501 Not Implemented",
            Status::ServiceUnavailable => "503 Service Unavailable",
            Status::HttpVersionNotSupported => "505 HTTP Version Not Supported",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}
