use crate::{Error, HeaderMap, Status, Version};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Response<'a> {
    version: Version,
    status: Status,
    header: HeaderMap<'a>,
    body: &'a [u8],
}

impl<'a> Response<'a> {
    pub fn version(&self) -> Version {
        self.version
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn header(&self) -> &HeaderMap<'a> {
        &self.header
    }

    pub fn body(&self) -> &'a [u8] {
        self.body
    }


    /// creates a response from bytes
    ///
    /// this function is usefull for converting a byte buffer received from a TcpStream into an
    /// actual http response which can be further handled
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
    /// use reqse::{Response, Version, Status};
    /// let raw_response = b"HTTP/1.1 200 OK\r\n\r\n";
    /// let response = Response::from_bytes(raw_response).unwrap();
    ///
    /// assert_eq!(response.version(), Version::Http1);
    /// assert!(response.body().is_empty());
    /// assert_eq!(response.status(), Status::Ok);
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

        let (version, status) = request_line.split_once(" ").ok_or(Error::InvalidHeader)?;

        let version: Version = version.trim().parse()?;
        let status: Status = status.trim().parse()?;

        let header = HeaderMap::new(header)?;

        let content_len: usize = header
            .get("Content-Length")
            .unwrap_or("0")
            .parse()
            .ok()
            .ok_or(Error::InvalidHeader)?;

        if content_len < body.len() {
            return Err(Error::NotEnoughData);
        }

        let body = &body[..content_len];

        Ok(Response {
            version,
            status,
            header,
            body,
        })
    }
}
