use std::process::Command;
use std::process::Output;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rustc_serialize::json;

use vgmdb::errors::*;
use vgmdb::data::*;

pub fn get_album(id: i32) -> VgResult<Album>{
    let html = try!( download_album_url(id) );
    let album : AlbumDb = try!( json::decode(&html));
    println!("Json {:?}", album);
    Ok(album.parse_tracks())
}

impl AlbumDb {
    fn parse_tracks (self) -> Album {
        let new_discs : Vec<Disc>  = self.discs
            .into_iter().map(  convert_disc  ).collect();
        return Album{ release_date : self.release_date
                    , discs:  new_discs
                    , catalog : self.catalog
                    , category : self.category
                    , classification : self.classification
                    , name: self.name
                    };
    }
}

fn download_album_url(id: i32) -> VgResult<String>{
    let url = format!("http://vgmdb.info/album/{}?format=json",id);
    println!("{}",url);
    let Output{status, stdout, .. } = try!( Command::new("curl").arg(&url).output()  );

    if ! status.success(){
        return Err(CmdError::OtherError(format!("Error Code {} on curl {}", status, url  ) ));
    };

    let res = try!(String::from_utf8(stdout));
    return Ok(res);
}

fn convert_disc( d: DiscDb ) -> Disc {
   Disc{ name: d.name
       , disc_length: convert_time(d.disc_length)
       , tracks: d.tracks.into_iter().zip((1..)).map(|(t,i)|  convert_track(t, i) ).collect()
       }
}

fn convert_track( t : TrackDb, index : i32 ) -> Track {

    let length = convert_time(t.track_length);
    let a  = [ t.names.English, t.names.Romaji, t.names.Japanese  ]
             .iter_mut().filter_map(|x| x.take()).nth(0).unwrap();

    Track{name:a, track_length:length, index:index }
}

fn convert_time(time: String) -> i32 {
    let mut length = 0;
    let mut mult = 1;

    for part in ( time.rsplit(":") ) {
        // println!("part {}", part  );
        length += mult * part.parse::<i32>().unwrap_or(0);
        mult *= 60;
    };
    return length;
}


