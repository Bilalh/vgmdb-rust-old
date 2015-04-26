mod vgmdb;

extern crate id3;
use id3::Tag;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;



extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct VgmdbResult {
    release_date: String
}


fn print_id3_tags(tag:Tag){
    println!("{}", tag.artist().unwrap() );
    println!("{}", tag.title().unwrap() );
}

fn decode_result(path:&'static str){
    let path = Path::new(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {
            let decoded : VgmdbResult  = json::decode(&s).unwrap();
            print!("decoded:{:?}", decoded)
        }
    }


}


fn main() {
    let mut tag = Tag::read_from_path("mp3s_examples/03 Class__XIO_PROCEED;.mp3");

    let _  = match tag {
       Ok(t)    =>  print_id3_tags(t),
       Err(_)   =>  ()
    };


    match vgmdb::io::get_url() {
        Some(x) => decode_result(x),
        None    => ()
    }


}

