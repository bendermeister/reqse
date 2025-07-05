use crate::{Error, HeaderMap, Method, Version};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Request<'a> {
    version: Version,
    uri: &'a str,
    method: Method,
    header: HeaderMap<'a>,
    body: &'a [u8],
}

impl<'a> Request<'a> {
    pub fn version(&self) -> Version {
        self.version
    }

    pub fn uri(&self) -> &'a str {
        self.uri
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn header(&self) -> &HeaderMap<'a> {
        &self.header
    }

    pub fn body(&self) -> &'a [u8] {
        self.body
    }

    /// creates a request from bytes
    ///
    /// this function is usefull for converting a byte buffer received from a TcpStream into an
    /// actual http request which can be further handled
    ///
    /// # Error
    /// - `Error::NotEnoughData` is returned when the passed `buffer: &[u8]` does not contain the
    ///   full request
    /// - `Error::InvalidUtf8` is returned when the http header is not valid utf-8
    /// - `Error::InvalidHeader` is returned when there is some other fuckup in the header (eg:
    ///   header is not formatted correctly)
    ///
    /// # Example
    /// ```
    /// use reqse::{Request, Method, Version};
    /// let raw_request = b"GET / HTTP/1.1\r\n\r\n";
    /// let request = Request::from_bytes(raw_request).unwrap();
    /// assert_eq!(request.method(), Method::Get);
    /// assert_eq!(request.uri(), "/");
    /// assert_eq!(request.version(), Version::Http1);
    /// assert!(request.body().is_empty());
    /// ```
    pub fn from_bytes(buf: &'a [u8]) -> Result<Self, Error> {
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

        let mut request_line = request_line.split_whitespace();

        let method: Method = request_line.next().ok_or(Error::InvalidHeader)?.parse()?;
        let uri = request_line.next().ok_or(Error::InvalidHeader)?;
        let version: Version = request_line.next().ok_or(Error::InvalidHeader)?.parse()?;

        if request_line.next().is_some() {
            return Err(Error::InvalidHeader);
        }

        let header = HeaderMap::new(header)?;

        let content_len: usize = header
            .get("Content-Length")
            .unwrap_or("0")
            .parse()
            .ok()
            .ok_or(Error::InvalidHeader)?;

        if body.len() < content_len {
            return Err(Error::NotEnoughData);
        }

        let body = &body[..content_len];

        Ok(Request {
            version,
            uri,
            method,
            header,
            body,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_bytes_1() {
        let raw_request = b"GET / HTTP/1.1\r\n\r\n";
        let request = Request::from_bytes(raw_request).unwrap();

        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.uri(), "/");
        assert_eq!(request.version(), Version::Http1);
        assert!(request.body().is_empty());
    }
}
