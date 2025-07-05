use crate::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HeaderMapIter<'a> {
    inner: &'a str,
}

impl<'a> Iterator for HeaderMapIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }
        let (line, rest) = self.inner.split_once("\r\n").unwrap_or((self.inner, ""));
        self.inner = rest;
        line.split_once("\r\n")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HeaderMap<'a> {
    inner: &'a str,
}

impl<'a> HeaderMap<'a> {
    pub fn new(header: &'a str) -> Result<Self, Error> {
        let header = header.trim();
        let err = header
            .split("\r\n")
            .filter(|line| !line.is_empty())
            .map(|line| line.split_once(": "))
            .find(|line| line.is_none());

        if err.is_some() {
            return Err(Error::InvalidHeader);
        }

        Ok(Self { inner: header })
    }

    pub fn iter(&self) -> HeaderMapIter<'a> {
        HeaderMapIter { inner: self.inner }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn get(&self, key: &str) -> Option<&'a str> {
        self.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}
