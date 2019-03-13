use std::error;
use std::fmt;
use std::io;
use std::result;
use std::sync::mpsc;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UserExit,
    NoBatteries,
    Battery(battery::Error),
    Io(io::Error),
    Channel(mpsc::RecvError),
    Logger(log::SetLoggerError),
    ParseError,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Battery(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Channel(e) => Some(e),
            Error::Logger(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UserExit => f.write_str("User-requested exit"),
            Error::NoBatteries => f.write_str("Unable to find any batteries installed"),
            Error::ParseError => f.write_str("Unable to parse value"),
            Error::Battery(e) => fmt::Display::fmt(e, f),
            Error::Io(e) => fmt::Display::fmt(e, f),
            Error::Channel(e) => fmt::Display::fmt(e, f),
            Error::Logger(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<battery::Error> for Error {
    fn from(e: battery::Error) -> Self {
        Error::Battery(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<mpsc::RecvError> for Error {
    fn from(e: mpsc::RecvError) -> Self {
        Error::Channel(e)
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(e: log::SetLoggerError) -> Self {
        Error::Logger(e)
    }
}
