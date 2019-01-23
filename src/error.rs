#[derive(Debug)]
pub enum ErrorCause {
    StdIo(std::io::Error),
    Http(http::Error),
}

#[derive(Debug)]
pub enum ErrorKind {
    Generic,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    cause: Option<ErrorCause>,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        println!("ERROR: {}", err);

        Error {
            kind: ErrorKind::Generic,
            cause: Some(ErrorCause::StdIo(err)),
        }
    }
}

impl std::convert::From<http::Error> for Error {
    fn from(err: http::Error) -> Self {
        println!("ERROR: {}", err);

        Error {
            kind: ErrorKind::Generic,
            cause: Some(ErrorCause::Http(err)),
        }
    }
}
