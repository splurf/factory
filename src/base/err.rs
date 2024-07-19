pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub enum ErrorKind {
    ConnectionFailed,
    Unexpected,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ConnectionFailed => "Failed to connect to API",
            Self::Unexpected => "An unexpected error has occurred",
        })
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Self::Misc(value)
    }
}

pub enum Error {
    Http(reqwest::Error),
    Api(serenity::Error),
    Misc(ErrorKind),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

impl From<serenity::Error> for Error {
    fn from(value: serenity::Error) -> Self {
        Self::Api(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Http(e) => e.to_string(),
            Self::Api(e) => e.to_string(),
            Self::Misc(e) => e.to_string(),
        })
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
