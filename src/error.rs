use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidHeader,
    InvalidUtf8,
    NotEnoughData,
}

impl Error {
    /// returns a `&'static str` represenation of the error
    ///
    /// # Examples
    /// ```
    /// use reqse::Error;
    ///
    /// let err = Error::InvalidHeader;
    /// assert_eq!("invalid header", err.to_static_str());
    /// ```
    pub fn to_static_str(&self) -> &'static str {
        match self {
            Error::InvalidHeader => "invalid header",
            Error::InvalidUtf8 => "invalid utf-8",
            Error::NotEnoughData => "not enough data",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}
