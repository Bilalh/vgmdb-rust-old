extern crate id3;

use id3::Tag;

fn print_id3_tags(tag:Tag){
    print!("{}", tag.artist().unwrap() );
}

fn main() {
    let mut tag = Tag::read_from_path("mp3s_examples/03 Class__XIO_PROCEED;.mp3");

    let _  = match tag {
       Ok(t)    =>  print_id3_tags(t),
       Err(_)   =>  ()
    };

}
