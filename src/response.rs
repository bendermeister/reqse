use std::collections::HashMap;

use crate::{Error, Status, Version};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Response {
    pub version: Version,
    pub status: Status,
    pub header: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ResponseBuilder {
    pub version: Option<Version>,
    pub status: Status,
    pub header: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl ResponseBuilder {
    /// creates a new response builder
    ///
    /// # Example
    /// ```
    /// use reqse::{ResponseBuilder, Status, Version};
    ///
    /// let builder = ResponseBuilder::new(Status::Ok);
    ///
    /// // the builder can now be used like this
    ///
    /// let response = builder
    ///     .version(Version::Http11)
    ///     .header("Key".to_owned(), "Value".to_owned())
    ///     .body("Hello World".as_bytes().to_owned())
    ///     .finish();
    /// ```
    pub fn new(status: Status) -> Self {
        Self {
            version: None,
            status,
            header: HashMap::new(),
            body: None,
        }
    }

    /// sets the http version of the currently building response
    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    /// sets a header of the currently building response
    pub fn header(mut self, key: String, value: String) -> Self {
        self.header.insert(key, value);
        self
    }

    /// sets the body (and the Content-Length) header for a currently building response
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// finishes building the response by consuming the builder into a response
    pub fn finish(self) -> Response {
        let version = self.version.unwrap_or(Version::Http11);
        let mut header = self.header;
        let status = self.status;
        let body = self.body;

        if let Some(len) = body.as_ref().map(|b| b.len()) {
            header.insert("Content-Length".to_owned(), len.to_string());
        } else {
            header.remove("Content-Length");
        }

        Response {
            version,
            status,
            header,
            body,
        }
    }
}

impl Response {
    /// returns a new builder to start building a response with status code `status`
    /// this is equivalent to `ResponseBuilder::new(status)`
    ///
    /// # Example
    /// ```
    /// use reqse::{ResponseBuilder, Status, Version};
    ///
    /// let builder = ResponseBuilder::new(Status::Ok);
    ///
    /// // the builder can now be used like this
    ///
    /// let response = builder
    ///     .version(Version::Http11)
    ///     .header("Key".to_owned(), "Value".to_owned())
    ///     .body("Hello World".as_bytes().to_owned())
    ///     .finish();
    /// ```
    pub fn builder(status: Status) -> ResponseBuilder {
        ResponseBuilder::new(status)
    }

    /// returns a new builder to build a response with status `200 Ok`
    pub fn ok() -> ResponseBuilder {
        Self::builder(Status::Ok)
    }

    /// returns a new builder to build a response with status `400 Bad Request`
    pub fn bad_request() -> ResponseBuilder {
        Self::builder(Status::BadRequest)
    }

    /// returns a new builder to build a response with status `401 Unauthorized`
    pub fn unauthorized() -> ResponseBuilder {
        Self::builder(Status::Unauthorized)
    }

    /// returns a new builder to build a response with status `403 Forbidden`
    pub fn forbidden() -> ResponseBuilder {
        Self::builder(Status::Forbidden)
    }

    /// returns a new builder to build a response with status `404 Not Found`
    pub fn not_found() -> ResponseBuilder {
        Self::builder(Status::NotFound)
    }

    /// returns a new builder to build a response with status `405 Method Not Allowed`
    pub fn method_not_allowed() -> ResponseBuilder {
        Self::builder(Status::MethodNotAllowed)
    }

    /// returns a new builder to build a response with status `500 Internal Server Error`
    pub fn internal_server_error() -> ResponseBuilder {
        Self::builder(Status::InternalServerError)
    }

    /// creates a response from bytes
    ///
    /// this function is usefull for converting a byte buffer received from a TcpStream into an
    /// actual http response which can be further handled
    ///
    /// # Error
    /// - `Error::NotEnoughData` is returned when the passed `buffer: &[u8]` does not contain the
    ///    full request
    /// - `Error::InvalidUtf8` is returned when the http header is not valid utf-8
    /// - `Error::InvalidHeader` is returned when there is some other fuckup in the header (eg:
    ///    header is not formatted correctly)
    ///
    /// # Example
    /// ```
    /// use reqse::Response;
    /// let raw_response = b"HTTP/1.1 200 OK\r\n\r\n";
    /// let request = Response::from_bytes(raw_response).unwrap();
    /// assert_eq!(Response::ok().finish(), request);
    /// ```
    pub fn from_bytes(buf: &[u8]) -> Result<Self, Error> {
        let mid = buf
            .windows(4)
            .enumerate()
            .find(|(_, w)| matches!(*w, b"\r\n\r\n"))
            .map(|(i, _)| i + 4)
            .ok_or(Error::NotEnoughData)?;

        let header = &buf[..mid];
        let body = &buf[mid..];

        let header = std::str::from_utf8(header).ok().ok_or(Error::InvalidUtf8)?;

        let (request_line, header) = header.split_once("\r\n").ok_or(Error::InvalidHeader)?;

        let (version, status) = request_line.split_once(" ").ok_or(Error::InvalidHeader)?;

        let version: Version = version.trim().parse()?;
        let status: Status = status.trim().parse()?;

        let header = header
            .trim()
            .split("\r\n")
            .filter(|line| !line.is_empty())
            .map(|header| header.split_once(": ").ok_or(Error::InvalidHeader))
            .collect::<Result<HashMap<_, _>, _>>()?;

        let content_len: usize = header
            .get("Content-Length")
            .unwrap_or(&"0")
            .parse()
            .ok()
            .ok_or(Error::InvalidHeader)?;

        if content_len < body.len() {
            return Err(Error::NotEnoughData);
        }

        let body = if content_len > 0 {
            Some(&body[..content_len])
        } else {
            None
        };

        Ok(Response {
            version,
            status,
            header: header
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
            body: body.map(|inner| inner.to_owned()),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![];

        buf.extend_from_slice(self.version.to_static().as_bytes());
        buf.push(b' ');
        buf.extend_from_slice(self.status.to_static_str().as_bytes());

        for (key, value) in self.header.iter() {
            buf.extend_from_slice(b"\r\n");
            buf.extend_from_slice(key.as_bytes());
            buf.extend_from_slice(b": ");
            buf.extend_from_slice(value.as_bytes());
        }
        buf.extend_from_slice(b"\r\n\r\n");

        if let Some(body) = self.body.as_deref() {
            buf.extend_from_slice(body);
        }

        buf
    }
}
