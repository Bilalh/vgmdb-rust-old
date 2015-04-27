use std::process::Command;
use std::process::Output;

use std::error::Error;
use std::string::FromUtf8Error;
use std::fmt;
use std::io;

use vgmdb::errors::*;

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

