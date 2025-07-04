use crate::{Error, Method, Version};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Request {
    pub version: Version,
    pub uri: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RequestBuilder {
    pub version: Option<Version>,
    pub uri: Option<String>,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl RequestBuilder {
    pub fn new(method: Method) -> Self {
        Self {
            version: None,
            uri: None,
            method,
            headers: HashMap::new(),
            body: None,
        }
    }

    /// sets http version or request
    ///
    /// # Examples
    /// ```
    /// use reqse::{Request, Version};
    ///
    /// let builder = Request::get()
    ///     .version(Version::Http11);
    /// ```
    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    /// sets uri of request
    ///
    /// # Examples
    /// ```
    /// use reqse::Request;
    ///
    /// let builder = Request::get()
    ///     .uri("/uri/to/some/cool/place".to_owned());
    /// ```
    pub fn uri(mut self, uri: String) -> Self {
        self.uri = Some(uri);
        self
    }

    /// inserts the `key` `value` pair into header values
    ///
    /// # Examples
    /// ```
    /// use reqse::Request;
    ///
    /// let builder = Request::get()
    ///     .header("Accept".to_owned(), "*/*".to_owned());
    /// ```
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// sets body of request
    ///
    /// # Examples
    /// ```
    /// use reqse::Request;
    ///
    /// let builder = Request::get()
    ///     .body(b"Hello World".to_vec());
    /// ```
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// consumes the builder to build the request
    ///
    /// # Error
    /// returns `reqse::Error::InvalidHeader` if method was not set
    pub fn finish(self) -> Request {
        let uri = self.uri.unwrap_or_else(|| "/".to_owned());
        let method = self.method;
        let version = self.version.unwrap_or(Version::Http11);
        let body = self.body.unwrap_or_else(|| vec![]);
        let mut headers = self.headers;

        if body.is_empty() {
            headers.remove("Content-Length");
        } else {
            headers.insert("Content-Length".to_owned(), body.len().to_string());
        }

        Request {
            version,
            uri,
            method,
            headers,
            body,
        }
    }
}

impl Request {
    /// returns a new request builder
    ///
    /// # Examples
    /// ```
    /// use reqse::{Request, Method};
    /// let request = Request::builder(Method::Get)
    ///     .uri("/uri/to/a/very/cool/place".to_owned())
    ///     .finish();
    /// ```
    pub fn builder(method: Method) -> RequestBuilder {
        RequestBuilder::new(method)
    }

    /// returns a new get request builder
    ///
    /// # Examples
    /// ```
    /// use reqse::{Request, Version, Method};
    ///
    /// let request = Request::get()
    ///     .uri("/uri/to/some/cool/place".to_owned())
    ///     .finish();
    ///
    /// ```
    pub fn get() -> RequestBuilder {
        Self::builder(Method::Get)
    }

    /// returns a new post request builder
    ///
    /// # Examples
    /// ```
    /// use reqse::{Request, Version, Method};
    ///
    /// let request = Request::get()
    ///     .uri("/uri/to/some/cool/place".to_owned())
    ///     .finish();
    ///
    /// ```
    pub fn post() -> RequestBuilder {
        Self::builder(Method::Post)
    }

    /// returns a new put request builder
    ///
    /// # Examples
    /// ```
    /// use reqse::{Request, Version, Method};
    ///
    /// let request = Request::get()
    ///     .uri("/uri/to/some/cool/place".to_owned())
    ///     .finish();
    ///
    /// ```
    pub fn put() -> RequestBuilder {
        Self::builder(Method::Put)
    }

    /// returns a new delete request builder
    ///
    /// # Examples
    /// ```
    /// use reqse::{Request, Version, Method};
    ///
    /// let request = Request::get()
    ///     .uri("/uri/to/some/cool/place".to_owned())
    ///     .finish();
    ///
    /// ```
    pub fn delete() -> RequestBuilder {
        Self::builder(Method::Delete)
    }

    /// creates a request from bytes
    ///
    /// this function is usefull for converting a byte buffer received from a TcpStream into an
    /// actual http request which can be further handled
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
    /// use reqse::Request;
    /// let raw_request = b"GET / HTTP/1.1\r\n\r\n";
    /// let request = Request::from_bytes(raw_request).unwrap();
    /// assert_eq!(Request::get().finish(), request);
    /// ```
    pub fn from_bytes(buffer: &[u8]) -> Result<Request, Error> {
        let mid = buffer
            .windows(4)
            .enumerate()
            .find(|(_, w)| matches!(*w, b"\r\n\r\n"))
            .map(|(i, _)| i + 4)
            .ok_or(Error::NotEnoughData)?;

        let header = &buffer[..mid];
        let body = &buffer[mid..];

        let header = std::str::from_utf8(header).ok().ok_or(Error::InvalidUtf8)?;

        let (request_line, headers) = header.split_once("\r\n").ok_or(Error::InvalidHeader)?;

        let mut request_line = request_line.split_whitespace();

        let method: Method = request_line.next().ok_or(Error::InvalidHeader)?.parse()?;
        let uri = request_line.next().ok_or(Error::InvalidHeader)?;
        let version: Version = request_line.next().ok_or(Error::InvalidHeader)?.parse()?;

        if request_line.next().is_some() {
            return Err(Error::InvalidHeader);
        }

        let headers = headers
            .trim()
            .split("\r\n")
            .filter(|line| !line.is_empty())
            .map(|header| header.split_once(": ").ok_or(Error::InvalidHeader))
            .collect::<Result<HashMap<_, _>, _>>()?;

        let content_len: usize = headers
            .get("Content-Length")
            .unwrap_or(&"0")
            .parse()
            .ok()
            .ok_or(Error::InvalidHeader)?;

        if content_len < body.len() {
            return Err(Error::NotEnoughData);
        }

        let body = &body[..content_len];

        Ok(Request {
            version,
            uri: uri.to_owned(),
            method,
            headers: headers
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
            body: body.to_owned(),
        })
    }

    /// converts a request to bytes
    ///
    /// this function is useful to send a http request over a TcpStream
    ///
    /// # Examples
    /// ```
    /// use reqse::Request;
    ///
    /// let request = Request::get().finish().to_bytes();
    /// let expected = b"GET / HTTP/1.1\r\n\r\n";
    ///
    /// assert_eq!(&request[..], &expected[..]);
    /// ```
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];

        buffer.extend_from_slice(self.method.to_static_str().as_bytes());
        buffer.push(b' ');
        buffer.extend_from_slice(self.uri.as_bytes());
        buffer.push(b' ');
        buffer.extend_from_slice(self.version.to_static().as_bytes());

        for (key, value) in self.headers.iter() {
            buffer.extend_from_slice(b"\r\n");
            buffer.extend_from_slice(key.as_bytes());
            buffer.extend_from_slice(b": ");
            buffer.extend_from_slice(value.as_bytes());
        }

        buffer.extend_from_slice(b"\r\n\r\n");
        buffer.extend_from_slice(&self.body);

        buffer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_bytes_1() {
        let raw_request = b"GET / HTTP/1.1\r\n\r\n";
        let request = Request::from_bytes(raw_request).unwrap();
        assert_eq!(Request::get().finish(), request);
        let bytes = request.to_bytes();
        assert_eq!(&raw_request[..], &bytes[..]);
    }
}
