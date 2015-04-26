use std::process::Command;

pub fn get_url() -> Option<&'static str> {

   match Command::new("wget").arg("http://vgmdb.info/album/79?format=json").status() {
        Ok(a)   =>  return Some("79?format=json"),
        Err(f)  =>  return None
    };

}

