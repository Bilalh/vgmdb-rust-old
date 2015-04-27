use std::process::Command;
use std::process::Output;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rustc_serialize::json;

use vgmdb::errors::*;
use vgmdb::data::*;

pub fn get_album(id: i32) -> VgResult<Album<Track>>{
    let html = try!( download_album_url(id) );
    let album : Album<TrackDb> = try!( json::decode(&html));
    Ok(album.parse_tracks())
}

impl Album<TrackDb> {
    fn parse_tracks (self) -> Album<Track> {
        let new_discs : Vec<Disc<Track>>  = self.discs
            .into_iter().map(  convert_disc  ).collect();
        return Album{ release_date : self.release_date
                    , discs:  new_discs
                    , catalog : self.catalog};
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

fn convert_disc( d: Disc<TrackDb> ) -> Disc<Track> {
   Disc{ name: d.name
       , disc_length: d.disc_length
       // , disc_length: convert_time(d.disc_length)
       , tracks: d.tracks.into_iter().map(convert_track).collect()
       }
}

fn convert_track( t : TrackDb) -> Track {

    let length = convert_time(t.track_length);
    let a  = [ t.names.English, t.names.Romaji, t.names.Japanese  ]
             .iter_mut().filter_map(|x| x.take()).nth(0).unwrap();

    Track{name:a, track_length:length }
}

fn convert_time(time: String) -> i32 {
    let mut length = 0;
    let mut mult = 1;

    for part in ( time.rsplit(":") ) {
        // println!("part {}", part  );
        length += mult * part.parse::<i32>().unwrap();
        mult *= 60;
    };
    return length;
}


pub fn decode_album(path:&'static str){
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
            let decoded : Album<TrackDb>  = json::decode(&s).unwrap();
            print!("decoded:{:?}", decoded)
        }
    }


}

