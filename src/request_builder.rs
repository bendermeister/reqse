use std::collections::HashMap;

use crate::{Method, Version};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RequestBuilder {
    method: Method,
    uri: String,
    version: Version,
    header: HashMap<String, String>,
    body: Vec<u8>,
}

impl RequestBuilder {
    pub fn new(method: Method, uri: String) -> Self {
        Self {
            method,
            uri,
            version: Version::default(),
            header: HashMap::default(),
            body: Vec::default(),
        }
    }

    pub fn get(uri: String) -> Self {
        Self::new(Method::Get, uri)
    }

    pub fn post(uri: String) -> Self {
        Self::new(Method::Post, uri)
    }

    pub fn put(uri: String) -> Self {
        Self::new(Method::Put, uri)
    }

    pub fn delete(uri: String) -> Self {
        Self::new(Method::Delete, uri)
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn uri_mut(&mut self) -> &mut String {
        &mut self.uri
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    pub fn header(&self) -> &HashMap<String, String> {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.header
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn body_but(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.body.is_empty() {
            self.header.remove("Content-Length");
        } else {
            self.header
                .insert("Content-Length".to_owned(), self.body.len().to_string());
        }

        let mut buf = vec![];

        buf.extend_from_slice(self.method.to_static_str().as_bytes());
        buf.push(b' ');

        let mut uri = self.uri.into_bytes();
        buf.append(&mut uri);

        buf.push(b' ');

        buf.extend_from_slice(self.version.to_static().as_bytes());

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
