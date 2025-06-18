use derive_more::From;
use rustpython_vm::builtins::PyBaseExceptionRef;
use std::fmt;

//////////////////////////////////////////////////////////////////
// Result
//////////////////////////////////////////////////////////////////

pub type Result<T = ()> = std::result::Result<T, Error>;

//////////////////////////////////////////////////////////////////
// Error
//////////////////////////////////////////////////////////////////

#[derive(From)]
pub enum Error {
    #[from]
    Text(String),

    #[from]
    Io(std::io::Error),

    #[from]
    Python(PyBaseExceptionRef),

    #[from]
    Json(serde_json::Error),
}

impl From<&str> for Error {
    fn from(text: &str) -> Self {
        Error::Text(text.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Text(text) => formatter.write_str(text),
            Error::Io(err) => formatter.write_fmt(format_args!("IO error: {}", err)),
            Error::Python(exception) => {
                formatter.write_str(format!("Python exception: {:?}", exception).as_str())
            }
            Error::Json(err) => formatter.write_str(format!("Json error: {}", err).as_str()),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, formatter)
    }
}
