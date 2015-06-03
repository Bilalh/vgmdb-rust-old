// Breakes on VBR files


mod vgmdb;

extern crate id3;
extern crate rustc_serialize;

use id3::Tag;
use std::fs;
use std::path::Path;

use std::ffi::OsStr;



fn print_id3_tags(tag:Tag){
    println!("{}", tag.artist().unwrap() );
    println!("{}", tag.title().unwrap() );
}


fn main() {

    let album = vgmdb::io::get_album(44046).unwrap();

    let dir       = "mp3s_examples/Ar nosurge Genometric Concert side.蒼〜刻神楽〜";
    let dir_paths = fs::read_dir(&Path::new(dir)).unwrap();

    let paths = dir_paths
        .filter_map(|x| x.ok())
        .filter(|x| x.path().extension().and_then(OsStr::to_str) == Some("mp3"));

    let tracks = album.tracks();
    let tracks_len = tracks.len();
    let discs_len = album.discs.len();

    for (path,(disc_num,track)) in paths.zip(tracks) {
        let p = path.path();
        let s = format!("{}", p.display());
        println!("Path: {}", s);
        println!("Data: {:?}", track );

        let comment = format!("{}\n{}\n{} - {}", ""
                             , album.catalog.clone().unwrap_or("".to_string())
                             , album.category.clone().unwrap_or("".to_string())
                             , album.classification.clone().unwrap_or("".to_string())
                             );

        let mut tag = Tag::read_from_path(p).unwrap();

        tag.set_title(track.name.clone());
        tag.set_track(track.index as u32);
        tag.set_total_tracks(tracks_len as u32);
        tag.set_disc(disc_num as u32);
        tag.set_total_discs(discs_len as u32);

        if let Some(ref date) = album.release_date{
            set_release_date(&mut tag,date.clone());
        }

        tag.remove_comment(None, None);
        tag.add_comment("", comment);

        tag.save();
        // set_year_id2_3(&mut tag,1393);

        println!("Processed: {}", s);
        println!("   ");
    }

}

fn set_year_id2_3(t:&mut Tag , year: i32) {
    t.add_text_frame("TDRC", format!("{}", year));
}

fn set_release_date(t:&mut Tag , date: String) {
    t.add_text_frame("TDRL", format!("{}", date));
}
