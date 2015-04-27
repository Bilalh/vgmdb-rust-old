mod vgmdb;

extern crate id3;
extern crate rustc_serialize;

use id3::Tag;

fn print_id3_tags(tag:Tag){
    println!("{}", tag.artist().unwrap() );
    println!("{}", tag.title().unwrap() );
}


fn main() {
    let tag = Tag::read_from_path("mp3s_examples/03 Class__XIO_PROCEED;.mp3");

    let _  = match tag {
       Ok(t)    =>  print_id3_tags(t),
       Err(_)   =>  ()
    };

    let album = vgmdb::io::get_album(79);
    println!("{:?}",album );

}

