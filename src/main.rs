mod vgmdb;

extern crate id3;
extern crate rustc_serialize;
extern crate argparse;


use id3::Tag;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

use argparse::{ArgumentParser,StoreFalse, Store};

#[derive(Debug)]
struct Options {
    length_check: bool,
    dir: String,
    album_id: i32
}

fn get_args() -> Options {
    let mut options = Options {
        length_check: true,
        dir: "".to_string(),
        album_id: 0
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Tags mp3s using vgmdb");
        ap.refer(&mut options.dir)
            .add_argument("dir", Store,
            "Directory of mp3s").required();
        ap.refer(&mut options.album_id)
            .add_argument("album_id", Store,
            "Id from from vgmdb").required();
        ap.refer(&mut options.length_check)
            .add_option(&["-l", "--no-length-check"], StoreFalse,
            "Continue even if there is mismatch in number of tracks  of the dir and the db");

        ap.parse_args_or_exit();
    }
    return options
}

fn main() {
    let options = get_args();
    println!("Args: {:?}", options);

    let album     = vgmdb::io::get_album(options.album_id).unwrap();
    let dir_paths = fs::read_dir    (&Path::new(&options.dir)).unwrap();

    let paths : Vec<_> = dir_paths
        .filter_map(|x| x.ok())
        .filter(|x| x.path().extension().and_then(OsStr::to_str) == Some("mp3"))
        .collect();

    let tracks = album.tracks();
    let tracks_len = tracks.len();
    let discs_len = album.discs.len();

    println!("Album: {:?}", album );

    let dir_len = paths.len();
    if options.length_check && dir_len != tracks_len{
        panic!("Lengths not equal, tracks {} != dir {} ", tracks_len, dir_len)
    }

    for (path,(disc_num,track)) in paths.iter().zip(tracks) {
        let p = path.path();
        let s = format!("{}", p.display());
        println!("Path: {}", s);
        println!("Data: {:?}", track );


        let mut buf = album.category.clone().unwrap_or("".to_string());
        if let Some(o) = album.classification.clone(){
            if buf.is_empty(){
                buf = o;
            }else{
                buf = format!("{}, {}",buf,o);
            }
        }

        let comment = format!(
               "\n{}, vgmdb.net/album/{}, \n{}"
             , album.catalog.clone().unwrap_or("".to_string())
             , options.album_id
             , buf );

        let mut tag = Tag::read_from_path(p).unwrap();

        tag.set_title(track.name.clone());
        tag.set_album(album.name.clone());

        tag.set_track(track.index as u32);
        tag.set_total_tracks(tracks_len as u32);
        tag.set_disc(disc_num as u32);
        tag.set_total_discs(discs_len as u32);

        if let Some(ref date) = album.release_date{
            set_release_date(&mut tag,date.clone());
            let year_str:String = date.chars().take(4).collect();

            if let Ok(year) = year_str.parse(){
                set_year_id2_3(&mut tag, year);
            }
        }


        tag.remove_comment(None, None);
        tag.add_comment("", comment);

        match  tag.save() {
           Ok(_)  => println!("Processed: {}", s),
           Err(e) => println!("Failed: {}, {}", s, e)
        }

        println!("   ");
    }

}


fn set_year_id2_3(t:&mut Tag , year: i32) {
    t.add_text_frame("TDRC", format!("{}", year));
}

fn set_release_date(t:&mut Tag , date: String) {
    t.add_text_frame("TDRL", format!("{}", date));
}
