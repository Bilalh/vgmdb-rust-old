use std::error::Error;
use std::string::FromUtf8Error;
use std::fmt;
use std::io;
use rustc_serialize::json;

pub type VgResult<A> = Result<A,CmdError>;

#[derive(Debug)]
pub enum CmdError {
    UtfError(FromUtf8Error),
    IoError(io::Error),
    OtherError(String)
}

impl From<FromUtf8Error> for CmdError {
    fn from(err: FromUtf8Error) -> CmdError {
        CmdError::UtfError(err)
    }
}

impl From<io::Error> for CmdError {
    fn from(err: io::Error) -> CmdError {
        CmdError::IoError(err)
    }
}

impl From<json::DecoderError> for CmdError {
    fn from(err: json::DecoderError) -> CmdError {
        CmdError::OtherError(format!("Json decode Error {:?}", err.description() ))
    }
}

impl Error for CmdError {
    fn description(&self) -> &str {
        match *self {
            CmdError::UtfError(ref err) => err.description(),
            CmdError::IoError(ref err) => err.description(),
            CmdError::OtherError(ref s) => s
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CmdError::UtfError(ref err) => Some(err as &Error),
            CmdError::IoError(ref err)  => Some(err as &Error),
            CmdError::OtherError(..)     => None
        }
    }
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CmdError::UtfError(ref err) => fmt::Display::fmt(err, f),
            CmdError::IoError(ref err)  => fmt::Display::fmt(err, f),
            CmdError::OtherError(ref s) => fmt::Display::fmt(s, f),
        }
    }
}



