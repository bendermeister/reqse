use std::collections::HashMap;

use crate::{Status, Version};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ResponseBuilder {
    version: Version,
    status: Status,
    header: HashMap<String, String>,
    body: Vec<u8>,
}

impl ResponseBuilder {
    pub fn new(status: Status) -> Self {
        Self {
            version: Version::default(),
            status,
            header: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn ok() -> Self {
        Self::new(Status::Ok)
    }

    pub fn bad_request() -> Self {
        Self::new(Status::BadRequest)
    }

    pub fn unauthorized() -> Self {
        Self::new(Status::Unauthorized)
    }

    pub fn forbidden() -> Self {
        Self::new(Status::Forbidden)
    }

    pub fn not_found() -> Self {
        Self::new(Status::NotFound)
    }

    pub fn internal_server_error() -> Self {
        Self::new(Status::InternalServerError)
    }

    pub fn header(&self) -> &HashMap<String, String> {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.header
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    pub fn finish(mut self) -> Vec<u8> {
        let mut buf = vec![];

        buf.extend_from_slice(self.version.to_static().as_bytes());
        buf.push(b' ');
        buf.extend_from_slice(self.status.to_static_str().as_bytes());

        if self.body.is_empty() {
            self.header.remove("Content-Length");
        } else {
            self.header
                .insert("Content-Length".to_owned(), self.body.len().to_string());
        }

        for (key, value) in self.header.into_iter() {
            let mut key = key.into_bytes();
            let mut value = value.into_bytes();

            buf.extend_from_slice(b"\r\n");
            buf.append(&mut key);
            buf.extend_from_slice(b": ");
            buf.append(&mut value);
        }

        buf.extend_from_slice(b"\r\n\r\n");
        buf.append(&mut self.body);

        buf
    }
}
