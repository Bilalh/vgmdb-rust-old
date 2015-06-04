pub type AlbumDb = AlbumT<DiscDb>;
pub type Album   = AlbumT<Disc>;

#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct AlbumT<T> {
      pub release_date: Option<String>
    , pub discs: Vec<T>
    , pub catalog: Option<String>
    , pub category: Option<String>
    , pub classification: Option<String>
    , pub name: String
}

#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct DiscDb{
      pub disc_length: String
    , pub name: String
    , pub tracks: Vec<TrackDb>
}

#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct Disc{
      pub disc_length: i32
    , pub name: String
    , pub tracks: Vec<Track>
}


#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct TrackDb{
      pub names: Names
    , pub track_length: String
}

#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct Track{
      pub name : String
    , pub track_length : i32
    , pub index : i32
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct Names{
      pub English:  Option<String>
    , pub Romaji:   Option<String>
    , pub Japanese: Option<String>
}


impl Album {

    pub fn tracks(&self) -> Vec<(i32, &Track)> {
        let mut v = vec![];
        for (d,i) in self.discs.iter().zip((1..)) {
            for t in &d.tracks {
                v.push((i,t));
            }
        }
        return v;
    }

}
