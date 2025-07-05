use std::{fmt::Display, str::FromStr};

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Version {
    Http0,
    #[default]
    Http1,
    Http2,
    Http3,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(Self::Http0),
            "HTTP/1.1" => Ok(Self::Http1),
            "HTTP/2" => Ok(Self::Http2),
            "HTTP/3" => Ok(Self::Http3),
            _ => Err(Error::InvalidHeader),
        }
    }
}

impl Version {
    pub fn to_static(&self) -> &'static str {
        match self {
            Version::Http0 => "HTTP/1.0",
            Version::Http1 => "HTTP/1.1",
            Version::Http2 => "HTTP/2",
            Version::Http3 => "HTTP/3",
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static())
    }
}
