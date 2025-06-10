use std::fmt;

pub type Result<T = ()> = std::result::Result<T, Error>;

pub struct Error {
    text: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl Error {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    pub fn from_error(e: impl std::error::Error) -> Self {
        Self {
            text: e.to_string(),
        }
    }

    pub fn from_io_error(e: &std::io::Error) -> Self {
        Self {
            text: e.to_string(),
        }
    }
}
