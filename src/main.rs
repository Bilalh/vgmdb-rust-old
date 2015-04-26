extern crate id3;
use id3::Tag;

extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct VgmdbResult {
    artist: String
}


fn print_id3_tags(tag:Tag){
    println!("{}", tag.artist().unwrap() );
    println!("{}", tag.title().unwrap() );
}

fn main() {
    let mut tag = Tag::read_from_path("mp3s_examples/03 Class__XIO_PROCEED;.mp3");

    let _  = match tag {
       Ok(t)    =>  print_id3_tags(t),
       Err(_)   =>  ()
    };

    let obj = VgmdbResult{
        artist: "Green World".to_string()
    };

    let encoded = json::encode(&obj).unwrap();
    println!("{}", encoded)

}
