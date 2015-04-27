
use std::process::Command;
use std::process::Output;

use std::error::Error;
use std::string::FromUtf8Error;
use std::fmt;
use std::io;

pub type VgResult<A> = Result<A,CmdError>;

#[derive(Debug)]
enum CmdError {
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


fn download_album_url(id: i32) -> VgResult<String>{
    let url = format!("http://vgmdb.info/album/{}?format=json",id);
    let Output{status, stdout, .. } = try!( Command::new("curl").arg(&url).output()  );

    if ! status.success(){
        return Err(CmdError::OtherError(format!("Error Code {} on curl {}", status, url  ) ));
    };

    let res = try!(String::from_utf8(stdout));
    return Ok(res);
}



pub fn get_url() -> Option<&'static str> {

   match Command::new("wget").arg("http://vgmdb.info/album/79?format=json").status() {
        Ok(a)   =>  return Some("79?format=json"),
        Err(f)  =>  return None
    };

}

