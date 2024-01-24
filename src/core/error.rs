use std::path::StripPrefixError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            message: message.into(),
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self { message: err }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<std::string::FromUtf16Error> for Error {
    fn from(err: std::string::FromUtf16Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<StripPrefixError> for Error {
    fn from(err: StripPrefixError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<walkdir::Error> for Error {
    fn from(value: walkdir::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
